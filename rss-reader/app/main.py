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


# ---- pages -----------------------------------------------------------------
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


# ---- articles api ----------------------------------------------------------
@app.get("/api/articles")
def list_articles(feed_id: int | None = None, unread: bool = False,
                  limit: int = 50, offset: int = 0):
    clauses, params = [], []
    if feed_id is not None:
        clauses.append("a.feed_id = ?")
        params.append(feed_id)
    if unread:
        clauses.append("a.read = 0")
    where = f"WHERE {' AND '.join(clauses)}" if clauses else ""
    params.extend([limit, offset])
    with db.connect() as conn:
        rows = conn.execute(
            f"""SELECT a.id, a.feed_id, a.title, a.link, a.author, a.summary,
                       a.published, a.read, f.title AS feed_title
                FROM articles a JOIN feeds f ON f.id = a.feed_id
                {where}
                ORDER BY a.published DESC, a.id DESC
                LIMIT ? OFFSET ?""",
            params,
        ).fetchall()
    return [_row_to_dict(r) for r in rows]


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
    return _row_to_dict(row)


app.mount("/static", StaticFiles(directory=STATIC_DIR), name="static")
