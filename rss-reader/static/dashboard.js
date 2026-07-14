const $ = (sel) => document.querySelector(sel);

function toast(msg, isErr = false) {
  const t = $("#toast");
  t.textContent = msg;
  t.className = "toast show" + (isErr ? " err" : "");
  setTimeout(() => (t.className = "toast"), 2800);
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
  return isNaN(d) ? iso : d.toLocaleDateString();
}

async function loadFeeds() {
  const feeds = await api("/api/feeds");
  $("#count").textContent = feeds.length;
  const el = $("#feedCards");
  el.innerHTML = "";
  if (feeds.length === 0) {
    el.innerHTML = '<div class="empty">No feeds yet — add one above.</div>';
    return;
  }
  feeds.forEach((f) => {
    const card = document.createElement("div");
    card.className = "feed-card";
    card.innerHTML =
      `<div class="info">
        <div class="name">${esc(f.title || "Untitled feed")}</div>
        <div class="url">${esc(f.url)}</div>
        ${f.last_error ? `<div class="err">⚠ ${esc(f.last_error)}</div>` : ""}
      </div>
      <span class="badge">${f.article_count} articles</span>
      <button class="danger" data-id="${f.id}">Delete</button>`;
    card.querySelector("button").onclick = () => removeFeed(f.id, f.title || f.url);
    el.appendChild(card);
  });
}

async function removeFeed(id, name) {
  if (!confirm(`Delete "${name}" and all its articles?`)) return;
  try {
    await api(`/api/feeds/${id}`, { method: "DELETE" });
    toast("Feed deleted.");
    loadFeeds();
  } catch (e) {
    toast(e.message, true);
  }
}

$("#addForm").onsubmit = async (e) => {
  e.preventDefault();
  const input = $("#urlInput");
  const btn = $("#addBtn");
  const url = input.value.trim();
  if (!url) return;
  btn.disabled = true;
  btn.textContent = "Adding…";
  try {
    const f = await api("/api/feeds", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ url }),
    });
    toast(`Added "${f.title || url}" — ${f.new_articles} articles.`);
    input.value = "";
    loadFeeds();
  } catch (e) {
    toast(e.message, true);
  } finally {
    btn.disabled = false;
    btn.textContent = "Add";
  }
};

loadFeeds();
