from contextlib import asynccontextmanager
from datetime import datetime, timezone
from pathlib import Path

from fastapi import FastAPI, HTTPException
from fastapi.responses import FileResponse
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel

from . import db, feeds

STATIC_DIR = Path(__file__).resolve().parent.parent / "static"


@asynccontextmanager
async def lifespan(app: FastAPI):
    db.init_db()
    yield


app = FastAPI(title="RSS Reader", lifespan=lifespan)


class FeedIn(BaseModel):
    url: str


def _row_to_dict(row) -> dict:
    return dict(row)


def _split_tags(value) -> list[str]:
    if not value:
        return []
    return [t for t in value.split(db.TAG_SEP) if t]


def _article(row) -> dict:
    d = dict(row)
    d["tags"] = _split_tags(d.get("tags"))
    return d


def _like_tag(tag: str) -> str:
    esc = tag.replace("\\", "\\\\").replace("%", "\\%").replace("_", "\\_")
    return f"%{db.TAG_SEP}{esc}{db.TAG_SEP}%"


# ---- pages -----------------------------------------------------------------
@app.get("/health")
def health():
    return {"status": "ok"}


@app.get("/")
def reader_page():
    return FileResponse(STATIC_DIR / "reader.html")


@app.get("/dashboard")
def dashboard_page():
    return FileResponse(STATIC_DIR / "dashboard.html")


# ---- feeds api -------------------------------------------------------------
@app.get("/api/feeds")
def list_feeds():
    with db.connect() as conn:
        rows = conn.execute(
            """SELECT f.*,
                      (SELECT COUNT(*) FROM articles a WHERE a.feed_id = f.id) AS article_count,
                      (SELECT COUNT(*) FROM articles a WHERE a.feed_id = f.id AND a.read = 0) AS unread_count
               FROM feeds f ORDER BY f.title COLLATE NOCASE"""
        ).fetchall()
    return [_row_to_dict(r) for r in rows]


@app.post("/api/feeds", status_code=201)
async def add_feed(feed: FeedIn):
    url = feed.url.strip()
    if not url.startswith(("http://", "https://")):
        raise HTTPException(400, "URL must start with http:// or https://")
    now = datetime.now(timezone.utc).isoformat()
    with db.connect() as conn:
        existing = conn.execute("SELECT id FROM feeds WHERE url = ?", (url,)).fetchone()
        if existing:
            raise HTTPException(409, "Feed already exists")
        cur = conn.execute(
            "INSERT INTO feeds (url, added_at) VALUES (?, ?)", (url, now)
        )
        feed_id = cur.lastrowid
    result = await feeds.fetch_feed(feed_id, url)
    if not result["ok"]:
        with db.connect() as conn:
            conn.execute("DELETE FROM feeds WHERE id = ?", (feed_id,))
        raise HTTPException(400, f"Could not read feed: {result['error']}")
    with db.connect() as conn:
        row = conn.execute("SELECT * FROM feeds WHERE id = ?", (feed_id,)).fetchone()
    return {**_row_to_dict(row), "new_articles": result["new"]}


@app.delete("/api/feeds/{feed_id}", status_code=204)
def delete_feed(feed_id: int):
    with db.connect() as conn:
        cur = conn.execute("DELETE FROM feeds WHERE id = ?", (feed_id,))
        if cur.rowcount == 0:
            raise HTTPException(404, "Feed not found")


@app.post("/api/refresh")
async def refresh_all():
    with db.connect() as conn:
        rows = conn.execute("SELECT id, url FROM feeds").fetchall()
    results = await feeds.refresh_feeds(rows)
    return {"refreshed": len(results), "results": results}


# ---- tags api --------------------------------------------------------------
@app.get("/api/tags")
def list_tags(feed_id: int | None = None):
    clause, params = ("WHERE feed_id = ?", [feed_id]) if feed_id is not None else ("", [])
    counts: dict[str, int] = {}
    with db.connect() as conn:
        rows = conn.execute(
            f"SELECT tags FROM articles {clause}", params
        ).fetchall()
    for r in rows:
        for tag in _split_tags(r["tags"]):
            counts[tag] = counts.get(tag, 0) + 1
    return [
        {"tag": t, "count": c}
        for t, c in sorted(counts.items(), key=lambda kv: (-kv[1], kv[0].lower()))
    ]


# ---- articles api ----------------------------------------------------------
@app.get("/api/articles")
def list_articles(feed_id: int | None = None, tag: str | None = None,
                  unread: bool = False, limit: int = 50, offset: int = 0):
    clauses, params = [], []
    if feed_id is not None:
        clauses.append("a.feed_id = ?")
        params.append(feed_id)
    if tag:
        clauses.append("a.tags LIKE ? ESCAPE '\\'")
        params.append(_like_tag(tag))
    if unread:
        clauses.append("a.read = 0")
    where = f"WHERE {' AND '.join(clauses)}" if clauses else ""
    params.extend([limit, offset])
    with db.connect() as conn:
        rows = conn.execute(
            f"""SELECT a.id, a.feed_id, a.title, a.link, a.author, a.summary,
                       a.published, a.read, a.tags, f.title AS feed_title
                FROM articles a JOIN feeds f ON f.id = a.feed_id
                {where}
                ORDER BY a.published DESC, a.id DESC
                LIMIT ? OFFSET ?""",
            params,
        ).fetchall()
    return [_article(r) for r in rows]


@app.get("/api/articles/{article_id}")
def get_article(article_id: int):
    with db.connect() as conn:
        row = conn.execute(
            """SELECT a.*, f.title AS feed_title FROM articles a
               JOIN feeds f ON f.id = a.feed_id WHERE a.id = ?""",
            (article_id,),
        ).fetchone()
        if row is None:
            raise HTTPException(404, "Article not found")
        conn.execute("UPDATE articles SET read = 1 WHERE id = ?", (article_id,))
    return _article(row)


app.mount("/static", StaticFiles(directory=STATIC_DIR), name="static")
