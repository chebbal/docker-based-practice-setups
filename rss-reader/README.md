# RSS Reader

A tiny self-hosted RSS/Atom reader. Two parts:

1. **Reader** (`/`) — three-pane view: feeds → articles → rendered content.
2. **Dashboard** (`/dashboard`) — add, validate, and remove feed URLs.

Feeds are fetched and parsed server-side (avoids browser CORS limits) and
cached in SQLite (`rss.db`). Articles are stored on add/refresh.

## Run

```sh
uv run rss
# or
uv run uvicorn app.main:app --reload
```

Then open http://127.0.0.1:8000

## Stack

FastAPI · feedparser · httpx · SQLite · vanilla JS frontend.
