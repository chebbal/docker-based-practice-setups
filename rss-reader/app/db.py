import sqlite3
from contextlib import contextmanager
from pathlib import Path

DB_PATH = Path(__file__).resolve().parent.parent / "rss.db"

SCHEMA = """
CREATE TABLE IF NOT EXISTS feeds (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    url          TEXT NOT NULL UNIQUE,
    title        TEXT,
    site_url     TEXT,
    added_at     TEXT NOT NULL,
    last_fetched TEXT,
    last_error   TEXT
);

CREATE TABLE IF NOT EXISTS articles (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    feed_id    INTEGER NOT NULL REFERENCES feeds(id) ON DELETE CASCADE,
    guid       TEXT NOT NULL,
    title      TEXT,
    link       TEXT,
    author     TEXT,
    summary    TEXT,
    content    TEXT,
    published  TEXT,
    fetched_at TEXT NOT NULL,
    read       INTEGER NOT NULL DEFAULT 0,
    UNIQUE(feed_id, guid)
);

CREATE INDEX IF NOT EXISTS idx_articles_feed ON articles(feed_id);
CREATE INDEX IF NOT EXISTS idx_articles_published ON articles(published DESC);
"""


def init_db() -> None:
    with connect() as conn:
        conn.executescript(SCHEMA)


@contextmanager
def connect():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA foreign_keys = ON")
    try:
        yield conn
        conn.commit()
    finally:
        conn.close()
