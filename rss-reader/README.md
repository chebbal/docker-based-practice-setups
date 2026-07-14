# RSS Reader

A tiny self-hosted RSS/Atom reader. Two parts:

1. **Reader** (`/`) — three-pane view: feeds → articles → rendered content.
2. **Dashboard** (`/dashboard`) — add, validate, and remove feed URLs.

Feeds are fetched and parsed server-side (avoids browser CORS limits) and
cached in SQLite (`rss.db`). Articles are stored on add/refresh.

## Run (local dev)

```sh
uv run rss
# or
uv run uvicorn app.main:app --reload
```

Then open http://127.0.0.1:8000

## Run with Docker

```sh
docker compose up -d --build
```

- Data lives in the `rss-data` volume at `/data/rss.db` — survives rebuilds and
  container recreation. `docker compose down` keeps it; `down -v` deletes it.
- By default the port is published on `127.0.0.1:8000` only, so the app is **not**
  exposed to the internet. Put a reverse proxy (TLS + auth) in front, or reach it
  over a private network/VPN, before opening it up.

### Deploy to a VPS

```sh
# on the server (Ubuntu/Debian, Docker + compose plugin installed)
git clone <repo> rss-reader && cd rss-reader
docker compose up -d --build
```

`RSS_DB_PATH` controls the SQLite location (defaults to `/data/rss.db` in the image).

## Stack

FastAPI · feedparser · httpx · SQLite · vanilla JS frontend. Containerized with a
uv-based multi-stage-friendly image and a healthcheck on `/health`.
