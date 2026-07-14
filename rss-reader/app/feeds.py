import asyncio
import ipaddress
import socket
from datetime import datetime, timezone
from urllib.parse import urlparse

import feedparser
import httpx

from . import db

USER_AGENT = "rss-reader/0.1 (+personal)"
MAX_REDIRECTS = 5


class FeedError(Exception):
    """Raised when a feed URL is unsafe or unfetchable."""


def _assert_public_url(url: str) -> None:
    """Reject non-http(s) schemes and any host that resolves to a
    private / loopback / link-local / reserved address (SSRF guard)."""
    parsed = urlparse(url)
    if parsed.scheme not in ("http", "https"):
        raise FeedError(f"Blocked URL scheme: {parsed.scheme or '(none)'}")
    host = parsed.hostname
    if not host:
        raise FeedError("URL has no host")
    try:
        infos = socket.getaddrinfo(host, parsed.port or 0, proto=socket.IPPROTO_TCP)
    except socket.gaierror:
        raise FeedError(f"Cannot resolve host: {host}")
    for info in infos:
        ip = ipaddress.ip_address(info[4][0])
        if (ip.is_private or ip.is_loopback or ip.is_link_local
                or ip.is_reserved or ip.is_multicast or ip.is_unspecified):
            # blocks 127.0.0.1, 10/172.16/192.168, 169.254.169.254 (cloud metadata), etc.
            raise FeedError(f"Blocked non-public address for {host}: {ip}")


def _now() -> str:
    return datetime.now(timezone.utc).isoformat()


def _entry_guid(entry) -> str:
    return entry.get("id") or entry.get("link") or entry.get("title", "")


def _entry_published(entry) -> str | None:
    parsed = entry.get("published_parsed") or entry.get("updated_parsed")
    if parsed:
        return datetime(*parsed[:6], tzinfo=timezone.utc).isoformat()
    return entry.get("published") or entry.get("updated")


def _entry_content(entry) -> str:
    if entry.get("content"):
        return entry["content"][0].get("value", "")
    return entry.get("summary", "")


def _entry_tags(entry) -> str | None:
    seen, tags = set(), []
    for t in entry.get("tags", []):
        term = (t.get("term") or "").strip()
        key = term.lower()
        if term and db.TAG_SEP not in term and key not in seen:
            seen.add(key)
            tags.append(term)
    if not tags:
        return None
    # surround-delimited (…\x1ftag\x1f…) so a tag matches with a plain LIKE
    return db.TAG_SEP + db.TAG_SEP.join(tags) + db.TAG_SEP


async def _fetch_bytes(url: str) -> bytes:
    # Follow redirects manually so every hop is re-validated against the SSRF
    # guard (a public URL could otherwise redirect to a private/metadata IP).
    async with httpx.AsyncClient(
        follow_redirects=False, timeout=15, headers={"User-Agent": USER_AGENT}
    ) as client:
        for _ in range(MAX_REDIRECTS + 1):
            await asyncio.to_thread(_assert_public_url, url)
            resp = await client.get(url)
            if resp.is_redirect:
                url = str(resp.url.join(resp.headers["location"]))
                continue
            resp.raise_for_status()
            return resp.content
    raise FeedError(f"Too many redirects (>{MAX_REDIRECTS})")


async def fetch_feed(feed_id: int, url: str) -> dict:
    """Fetch one feed, parse it, upsert its articles. Returns a status dict."""
    try:
        raw = await _fetch_bytes(url)
        parsed = await asyncio.to_thread(feedparser.parse, raw)
    except Exception as exc:  # network / parse failure
        with db.connect() as conn:
            conn.execute(
                "UPDATE feeds SET last_fetched = ?, last_error = ? WHERE id = ?",
                (_now(), str(exc), feed_id),
            )
        return {"feed_id": feed_id, "ok": False, "error": str(exc), "new": 0}

    if not parsed.version and not parsed.entries:
        msg = "Not a valid RSS/Atom feed"
        with db.connect() as conn:
            conn.execute(
                "UPDATE feeds SET last_fetched = ?, last_error = ? WHERE id = ?",
                (_now(), msg, feed_id),
            )
        return {"feed_id": feed_id, "ok": False, "error": msg, "new": 0}

    title = parsed.feed.get("title")
    site_url = parsed.feed.get("link")
    new_count = 0
    now = _now()
    with db.connect() as conn:
        conn.execute(
            "UPDATE feeds SET title = COALESCE(?, title), site_url = COALESCE(?, site_url), "
            "last_fetched = ?, last_error = NULL WHERE id = ?",
            (title, site_url, now, feed_id),
        )
        for entry in parsed.entries:
            cur = conn.execute(
                """INSERT OR IGNORE INTO articles
                   (feed_id, guid, title, link, author, summary, content, published, fetched_at, tags)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)""",
                (
                    feed_id,
                    _entry_guid(entry),
                    entry.get("title"),
                    entry.get("link"),
                    entry.get("author"),
                    entry.get("summary"),
                    _entry_content(entry),
                    _entry_published(entry),
                    now,
                    _entry_tags(entry),
                ),
            )
            new_count += cur.rowcount
    return {"feed_id": feed_id, "ok": True, "title": title, "new": new_count}


async def refresh_feeds(feed_rows) -> list[dict]:
    """Fetch many feeds concurrently."""
    tasks = [fetch_feed(row["id"], row["url"]) for row in feed_rows]
    return await asyncio.gather(*tasks)
