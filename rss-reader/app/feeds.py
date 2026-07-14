import asyncio
from datetime import datetime, timezone

import feedparser
import httpx

from . import db

USER_AGENT = "rss-reader/0.1 (+personal)"


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


async def _fetch_bytes(url: str) -> bytes:
    async with httpx.AsyncClient(
        follow_redirects=True, timeout=15, headers={"User-Agent": USER_AGENT}
    ) as client:
        resp = await client.get(url)
        resp.raise_for_status()
        return resp.content


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
                   (feed_id, guid, title, link, author, summary, content, published, fetched_at)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)""",
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
                ),
            )
            new_count += cur.rowcount
    return {"feed_id": feed_id, "ok": True, "title": title, "new": new_count}


async def refresh_feeds(feed_rows) -> list[dict]:
    """Fetch many feeds concurrently."""
    tasks = [fetch_feed(row["id"], row["url"]) for row in feed_rows]
    return await asyncio.gather(*tasks)
