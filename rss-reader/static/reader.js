const $ = (sel) => document.querySelector(sel);
let currentFeed = null; // null = all feeds
let currentArticle = null;

function toast(msg, isErr = false) {
  const t = $("#toast");
  t.textContent = msg;
  t.className = "toast show" + (isErr ? " err" : "");
  setTimeout(() => (t.className = "toast"), 2600);
}

async function api(path, opts) {
  const res = await fetch(path, opts);
  if (!res.ok) throw new Error((await res.json().catch(() => ({}))).detail || res.statusText);
  return res.status === 204 ? null : res.json();
}

function fmtDate(iso) {
  if (!iso) return "";
  const d = new Date(iso);
  if (isNaN(d)) return iso;
  return d.toLocaleDateString(undefined, { month: "short", day: "numeric", year: "numeric" });
}

async function loadFeeds() {
  const feeds = await api("/api/feeds");
  const el = $("#feedList");
  const total = feeds.reduce((s, f) => s + f.unread_count, 0);
  el.innerHTML = "";
  el.appendChild(feedRow({ id: null, title: "All feeds", unread_count: total }, currentFeed === null));
  feeds.forEach((f) => el.appendChild(feedRow(f, currentFeed === f.id)));
  if (feeds.length === 0)
    el.insertAdjacentHTML("beforeend", '<div class="empty">No feeds yet.<br><a href="/dashboard">Add one →</a></div>');
}

function feedRow(f, active) {
  const div = document.createElement("div");
  div.className = "feed-item" + (active ? " active" : "");
  div.innerHTML = `<div class="row"><span class="name">${esc(f.title || f.url || "Untitled")}</span>` +
    `${f.unread_count ? `<span class="badge">${f.unread_count}</span>` : ""}</div>`;
  div.onclick = () => { currentFeed = f.id; loadFeeds(); loadArticles(); };
  return div;
}

async function loadArticles() {
  const q = currentFeed === null ? "" : `?feed_id=${currentFeed}`;
  const articles = await api(`/api/articles${q}`);
  $("#listTitle").textContent = articles.length ? `Articles (${articles.length})` : "Articles";
  const el = $("#articleList");
  el.innerHTML = "";
  if (articles.length === 0) {
    el.innerHTML = '<div class="empty">Nothing here. Try Refresh.</div>';
    return;
  }
  articles.forEach((a) => {
    const div = document.createElement("div");
    div.className = "article-item " + (a.read ? "read" : "unread") + (currentArticle === a.id ? " active" : "");
    div.innerHTML = `<div class="a-title">${esc(a.title || "(untitled)")}</div>` +
      `<div class="a-meta">${esc(a.feed_title || "")} · ${fmtDate(a.published)}</div>`;
    div.onclick = () => openArticle(a.id);
    el.appendChild(div);
  });
}

async function openArticle(id) {
  currentArticle = id;
  const a = await api(`/api/articles/${id}`);
  const body = a.content || a.summary || "<p class='muted'>No content.</p>";
  $("#content").innerHTML =
    `<article class="article-full">
      <h1>${esc(a.title || "(untitled)")}</h1>
      <div class="a-meta">${esc(a.feed_title || "")}${a.author ? " · " + esc(a.author) : ""} · ${fmtDate(a.published)}
        ${a.link ? ` · <a href="${esc(a.link)}" target="_blank" rel="noopener">Open original ↗</a>` : ""}</div>
      <div class="article-body">${body}</div>
    </article>`;
  document.querySelectorAll(".article-item").forEach((n) => n.classList.remove("active"));
  loadArticles();
}

function esc(s) {
  return String(s ?? "").replace(/[&<>"']/g, (c) =>
    ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c]));
}

$("#refreshBtn").onclick = async () => {
  const btn = $("#refreshBtn");
  btn.disabled = true;
  btn.textContent = "↻ Refreshing…";
  try {
    const r = await api("/api/refresh", { method: "POST" });
    const added = r.results.reduce((s, x) => s + (x.new || 0), 0);
    toast(`Refreshed ${r.refreshed} feed(s), ${added} new article(s).`);
    await loadFeeds();
    await loadArticles();
  } catch (e) {
    toast(e.message, true);
  } finally {
    btn.disabled = false;
    btn.textContent = "↻ Refresh";
  }
};

loadFeeds();
loadArticles();
