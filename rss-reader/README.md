# RSS Reader

A tiny self-hosted RSS/Atom reader. Two parts:

1. **Reader** (`/`) — three-pane view: feeds → articles → rendered content, grouped
   by date, filterable by tag.
2. **Dashboard** (`/dashboard`) — add, validate, and remove feed URLs.

Feeds are fetched and parsed server-side (avoids browser CORS limits) and cached in
SQLite. Deleting a feed cascade-deletes its articles.

**Stack:** FastAPI · feedparser · httpx · SQLite · vanilla JS. Shipped as a minimal
(~234 MB), non-root, multi-stage Docker image with a `/health` healthcheck.

---

## Local development

```sh
uv run rss                              # http://127.0.0.1:8000
# or:  uv run uvicorn app.main:app --reload
```

The DB path is `RSS_DB_PATH` (defaults to `./rss.db` locally, `/data/rss.db` in the image).

---

## Production deploy (VPS + domain + HTTPS)

The compose stack runs two containers: the **app** (internal only, hardened) and
**Caddy**, which terminates TLS (automatic Let's Encrypt certs) and enforces HTTP
Basic auth before proxying to the app.

```
Internet ──443──► Caddy (TLS + basic auth) ──8000──► rss-reader ──► /data/rss.db (volume)
```

### 1. Prerequisites
- A VPS with a **public IP** and ports **80** + **443** open.
- Docker Engine + the Compose plugin installed.
- A domain/subdomain whose **A record points at the VPS's public IP** (AAAA for IPv6).
  Point it at the *public* IP — a private IP (10.x/172.16.x/192.168.x) is not routable
  from the internet. Verify: `dig +short rss.example.com` should return your VPS IP.

### 2. Harden the host (once)
```sh
git clone <repo> rss-reader && cd rss-reader
sudo ./scripts/harden-host.sh                 # ufw + fail2ban + auto security updates
# optional, only after confirming SSH key login works:
sudo HARDEN_SSH=1 ./scripts/harden-host.sh    # disable SSH password + root login
```

### 3. Configure secrets
```sh
cp .env.example .env
# generate a bcrypt password hash (with $ escaped to $$ for Compose):
docker run --rm caddy caddy hash-password --plaintext 'your-password' | sed 's/\$/$$/g'
```
Edit `.env` — set `DOMAIN`, `ACME_EMAIL`, `BASIC_AUTH_USER`, and paste the escaped
hash into `BASIC_AUTH_HASH`. `.env` is git-ignored.

### 4. Launch
```sh
docker compose up -d --build
docker compose ps                             # both services should be Up/healthy
docker compose logs -f caddy                  # watch the cert get issued
```
Open `https://your-domain` and log in. Done.

### 5. Updating
```sh
git pull
docker compose up -d --build                  # data volume is untouched
```

---

## Backups

The DB lives in the `rss-data` volume. Take a **consistent** hot backup with SQLite's
online backup (don't just `cp` the live file):

```sh
docker exec rss-reader sqlite3 /data/rss.db ".backup '/data/backup-$(date +%F).db'"
docker cp rss-reader:/data/backup-$(date +%F).db ./backup-$(date +%F).db
```

Cron example (daily 03:00, keep in `crontab -e`):
```
0 3 * * * cd /path/to/rss-reader && docker exec rss-reader sqlite3 /data/rss.db ".backup '/data/backup.db'" && docker cp rss-reader:/data/backup.db /srv/backups/rss-$(date +\%F).db
```

---

## Running privately instead (no domain / VPN or LAN)

Skip TLS/auth and reach it over Tailscale/VPN: comment out the `caddy` service in
`compose.yaml`, add a host port to `rss-reader`:
```yaml
    ports:
      - "127.0.0.1:8000:8000"   # or "<tailscale-ip>:8000:8000"
```
then `docker compose up -d rss-reader`. Only expose it on a trusted network.

---

## Security notes

- **TLS + auth**: Caddy provides automatic HTTPS with HSTS and gates the whole app
  behind HTTP Basic auth. The app itself is never published to the host.
- **SSRF guard**: the server fetches only feed URLs whose host resolves to a *public*
  address — loopback, private ranges, link-local and the `169.254.169.254` cloud
  metadata endpoint are blocked, and every redirect hop is re-checked
  (`app/feeds.py`). Residual: DNS-rebinding is not fully closed; auth is the backstop.
- **Container hardening**: runs as non-root (uid 999), read-only rootfs, `cap_drop:
  ALL`, `no-new-privileges`, memory/PID limits.
- **Firewall**: `harden-host.sh` allows only 22/80/443. Note Docker's published ports
  bypass ufw — this stack publishes nothing to the host, so you're covered; keep it
  that way or add `DOCKER-USER` rules.
- **Log rotation**: both containers cap logs at 3 × 10 MB.
- **Known gap — feed HTML is rendered unsanitized** in the reader. A malicious feed
  could run script in your browser (stored XSS). Low risk single-user-behind-auth;
  sanitize or sandbox before any multi-user/public exposure.
