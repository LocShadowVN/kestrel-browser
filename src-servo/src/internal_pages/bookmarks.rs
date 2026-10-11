//! Trang bookmark nội bộ — kestrel://bookmarks

use crate::internal_pages::NAV_SHIM;

pub fn render(records: &[(i64, String, String)]) -> String {
    let mut rows = String::new();
    for (id, url, title) in records {
        rows.push_str(&format!(
            r#"<li class="item">
  <a class="title" href="{url}">{title}</a>
  <span class="url">{url}</span>
  <button class="remove" data-id="{id}" title="Remove">×</button>
</li>"#,
            id = id,
            url = escape(url),
            title = escape(title),
        ));
    }
    if rows.is_empty() {
        rows.push_str(r#"<li class="empty">No bookmarks yet. Click the bookmark icon in the toolbar to save a page.</li>"#);
    }
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Kestrel — Bookmarks</title>
<style>
  :root {{ --bg:#fafafa; --fg:#1a1a1a; --accent:#0a84ff; --card:#fff; --border:#e5e5e5; --muted:#6b6b6b; }}
  @media (prefers-color-scheme: dark) {{
    :root {{ --bg:#1c1c1e; --fg:#f5f5f7; --card:#2c2c2e; --border:#3a3a3c; --muted:#8e8e93; }}
  }}
  * {{ box-sizing:border-box; margin:0; padding:0; }}
  body {{ font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;
          background:var(--bg); color:var(--fg); padding:48px 24px; }}
  .container {{ max-width:800px; margin:0 auto; }}
  h1 {{ font-size:28px; font-weight:700; margin-bottom:8px; }}
  .count {{ font-size:13px; color:var(--muted); margin-bottom:24px; }}
  ul {{ list-style:none; }}
  .item {{ display:flex; align-items:center; gap:12px; padding:14px 20px;
           background:var(--card); border:1px solid var(--border);
           border-radius:12px; margin-bottom:8px; }}
  .item .title {{ font-weight:600; color:var(--accent); text-decoration:none;
                  flex:0 0 auto; max-width:220px; overflow:hidden;
                  text-overflow:ellipsis; white-space:nowrap; }}
  .item .title:hover {{ text-decoration:underline; }}
  .item .url {{ color:var(--muted); font-size:12px; flex:1 1 auto;
                overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }}
  .item .remove {{ background:transparent; border:1px solid var(--border);
                   color:var(--fg); border-radius:6px; width:28px; height:28px;
                   cursor:pointer; font-size:16px; line-height:1; padding:0; }}
  .item .remove:hover {{ background:#c0392b; color:#fff; border-color:#c0392b; }}
  .empty {{ color:var(--muted); padding:32px; text-align:center;
            background:var(--card); border:1px solid var(--border);
            border-radius:12px; list-style:none; }}
</style>
</head>
<body>
  <div class="container">
    <h1>Bookmarks</h1>
    <div class="count">{count} items</div>
    <ul>{rows}</ul>
  </div>
  <script>
    {shim}
    document.addEventListener('click', function (e) {{
      var btn = e.target.closest('.remove');
      if (!btn) return;
      e.preventDefault();
      var id = btn.getAttribute('data-id');
      if (window.servoGtk && window.servoGtk.messageHandlers &&
          window.servoGtk.messageHandlers.kestrel) {{
        window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify({{
          action: 'remove-bookmark',
          id: parseInt(id, 10)
        }}));
        var li = btn.closest('.item');
        if (li) li.remove();
      }}
    }});
  </script>
</body>
</html>"#,
        count = records.len(),
        rows = rows,
        shim = NAV_SHIM,
    )
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
