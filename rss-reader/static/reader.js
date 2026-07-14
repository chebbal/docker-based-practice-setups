const $ = (sel) => document.querySelector(sel);
let currentFeed = null; // null = all feeds
let currentTag = null; // null = all tags
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

function esc(s) {
  return String(s ?? "").replace(/[&<>"']/g, (c) =>
    ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c]));
}

function fmtDate(iso) {
  if (!iso) return "";
  const d = new Date(iso);
  if (isNaN(d)) return iso;
  return d.toLocaleDateString(undefined, { month: "short", day: "numeric", year: "numeric" });
}

// Bucket label for grouping the article list by day.
function dateGroup(iso) {
  if (!iso) return "Undated";
  const d = new Date(iso);
  if (isNaN(d)) return "Undated";
  const today = new Date();
  const startOf = (x) => new Date(x.getFullYear(), x.getMonth(), x.getDate()).getTime();
  const days = Math.round((startOf(today) - startOf(d)) / 86400000);
  if (days <= 0) return "Today";
  if (days === 1) return "Yesterday";
  if (days < 7) return d.toLocaleDateString(undefined, { weekday: "long" });
  if (d.getFullYear() === today.getFullYear())
    return d.toLocaleDateString(undefined, { month: "long", day: "numeric" });
  return d.toLocaleDateString(undefined, { month: "long", day: "numeric", year: "numeric" });
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
  div.onclick = () => { currentFeed = f.id; currentTag = null; refresh(); };
  return div;
}

async function loadTags() {
  const q = currentFeed === null ? "" : `?feed_id=${currentFeed}`;
  const tags = await api(`/api/tags${q}`);
  const el = $("#tagList");
  el.innerHTML = "";
  if (tags.length === 0) {
    el.innerHTML = '<div class="empty" style="padding:16px">No tags in these articles.</div>';
    return;
  }
  tags.forEach((t) => {
    const div = document.createElement("div");
    div.className = "feed-item" + (currentTag === t.tag ? " active" : "");
    div.innerHTML = `<div class="row"><span class="name">#${esc(t.tag)}</span>` +
      `<span class="badge">${t.count}</span></div>`;
    div.onclick = () => { currentTag = currentTag === t.tag ? null : t.tag; refresh(); };
    el.appendChild(div);
  });
}

function renderFilterBar() {
  const bar = $("#filterBar");
  if (currentTag === null) {
    bar.hidden = true;
    bar.innerHTML = "";
    return;
  }
  bar.hidden = false;
  bar.innerHTML = `<span class="chip active">#${esc(currentTag)}` +
    `<button class="chip-x" title="Clear tag filter">×</button></span>`;
  bar.querySelector(".chip-x").onclick = () => { currentTag = null; refresh(); };
}

async function loadArticles() {
  const p = new URLSearchParams();
  if (currentFeed !== null) p.set("feed_id", currentFeed);
  if (currentTag !== null) p.set("tag", currentTag);
  const qs = p.toString();
  const articles = await api(`/api/articles${qs ? "?" + qs : ""}`);
  $("#listTitle").textContent = articles.length ? `Articles (${articles.length})` : "Articles";
  renderFilterBar();
  const el = $("#articleList");
  el.innerHTML = "";
  if (articles.length === 0) {
    el.innerHTML = '<div class="empty">Nothing here. Try Refresh.</div>';
    return;
  }
  let group = null;
  articles.forEach((a) => {
    const g = dateGroup(a.published);
    if (g !== group) {
      group = g;
      const h = document.createElement("div");
      h.className = "date-header";
      h.textContent = g;
      el.appendChild(h);
    }
    el.appendChild(articleRow(a));
  });
}

function articleRow(a) {
  const div = document.createElement("div");
  div.className = "article-item " + (a.read ? "read" : "unread") + (currentArticle === a.id ? " active" : "");
  const tags = (a.tags || []).slice(0, 3)
    .map((t) => `<span class="tag-chip" data-tag="${esc(t)}">#${esc(t)}</span>`).join("");
  div.innerHTML =
    `<div class="a-title">${esc(a.title || "(untitled)")}</div>` +
    `<div class="a-meta">${esc(a.feed_title || "")} · ${fmtDate(a.published)}</div>` +
    (tags ? `<div class="tag-row">${tags}</div>` : "");
  div.onclick = () => openArticle(a.id);
  div.querySelectorAll(".tag-chip").forEach((chip) => {
    chip.onclick = (e) => { e.stopPropagation(); currentTag = chip.dataset.tag; refresh(); };
  });
  return div;
}

async function openArticle(id) {
  currentArticle = id;
  const a = await api(`/api/articles/${id}`);
  const body = a.content || a.summary || "<p class='muted'>No content.</p>";
  const tags = (a.tags || [])
    .map((t) => `<span class="tag-chip" data-tag="${esc(t)}">#${esc(t)}</span>`).join(" ");
  $("#content").innerHTML =
    `<article class="article-full">
      <h1>${esc(a.title || "(untitled)")}</h1>
      <div class="a-meta">${esc(a.feed_title || "")}${a.author ? " · " + esc(a.author) : ""} · ${fmtDate(a.published)}
        ${a.link ? ` · <a href="${esc(a.link)}" target="_blank" rel="noopener">Open original ↗</a>` : ""}</div>
      ${tags ? `<div class="tag-row">${tags}</div>` : ""}
      <div class="article-body">${body}</div>
    </article>`;
  $("#content").querySelectorAll(".tag-chip").forEach((chip) => {
    chip.onclick = () => { currentTag = chip.dataset.tag; refresh(); };
  });
  loadArticles();
}

async function refresh() {
  await Promise.all([loadFeeds(), loadTags(), loadArticles()]);
}

$("#refreshBtn").onclick = async () => {
  const btn = $("#refreshBtn");
  btn.disabled = true;
  btn.textContent = "↻ Refreshing…";
  try {
    const r = await api("/api/refresh", { method: "POST" });
    const added = r.results.reduce((s, x) => s + (x.new || 0), 0);
    toast(`Refreshed ${r.refreshed} feed(s), ${added} new article(s).`);
    await refresh();
  } catch (e) {
    toast(e.message, true);
  } finally {
    btn.disabled = false;
    btn.textContent = "↻ Refresh";
  }
};

refresh();
