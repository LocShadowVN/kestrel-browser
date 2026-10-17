//! Trang bookmark nội bộ — kestrel://bookmarks (Đơn sắc, tối giản).

use crate::internal_pages::NAV_SHIM;

pub fn render(records: &[(i64, String, String)], lang: &str, dark: bool) -> String {
    let vi = lang == "vi";
    let theme_class = if dark { "dark" } else { "light" };
    let t = |en: &str, vn: &str| -> String {
        if vi { vn.to_string() } else { en.to_string() }
    };

    let mut rows = String::new();
    for (id, url, title) in records {
        rows.push_str(&format!(
            r#"<li class="item" data-search="{search_text}">
  <svg class="bm-icon" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"/></svg>
  <a class="title" href="{url}">{title}</a>
  <span class="url">{url}</span>
  <button class="remove" data-id="{id}" title="Remove">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
  </button>
</li>"#,
            id = id,
            url = escape(url),
            title = escape(title),
            search_text = escape(&format!("{} {}", title.to_lowercase(), url.to_lowercase())),
        ));
    }
    if rows.is_empty() {
        rows.push_str(&format!(
            r#"<li class="empty">{}</li>"#,
            t(
                "No bookmarks yet. Click the star icon in the address bar (or press Ctrl+D) to save a page.",
                "Chưa có trang đánh dấu nào. Nhấn biểu tượng ngôi sao trên thanh địa chỉ (hoặc Ctrl+D) để lưu trang."
            )
        ));
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="{lang}" class="{theme_class}">
<head>
<meta charset="utf-8">
<title>Kestrel — {heading}</title>
<style>
  :root {{
    --bg: #101116; --fg: #e6e8f0; --muted: #8a8f9e;
    --card: #171921; --border: #252834; --border-focus: #454a5e;
    --btn-bg: #e6e8f0; --btn-fg: #101116;
  }}
  html.light {{
    --bg: #f5f6f8; --fg: #16181f; --muted: #606575;
    --card: #ffffff; --border: #dfe2ea; --border-focus: #8a90a6;
    --btn-bg: #16181f; --btn-fg: #ffffff;
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Inter, Roboto, sans-serif;
    background: var(--bg); color: var(--fg); padding: 48px 24px;
  }}
  .container {{ max-width: 820px; margin: 0 auto; }}
  .top-bar {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }}
  h1 {{ font-size: 26px; font-weight: 700; letter-spacing: -0.4px; }}
  .count {{ font-size: 12.5px; color: var(--muted); margin-top: 2px; }}
  .add-form {{
    display: flex; gap: 10px; margin-bottom: 14px; flex-wrap: wrap;
    background: var(--card); border: 1px solid var(--border); padding: 12px; border-radius: 12px;
  }}
  .add-form input {{
    flex: 1 1 200px; padding: 8px 12px; border-radius: 8px;
    border: 1px solid var(--border); background: var(--bg); color: var(--fg); font-size: 13.5px; outline: none;
  }}
  .add-form input:focus {{ border-color: var(--border-focus); }}
  .add-form button {{
    padding: 8px 16px; border-radius: 8px; border: none;
    background: var(--btn-bg); color: var(--btn-fg); font-size: 13px; font-weight: 600; cursor: pointer;
  }}
  .filter-input {{
    width: 100%; padding: 10px 14px; margin-bottom: 14px; border-radius: 10px;
    border: 1px solid var(--border); background: var(--card); color: var(--fg); font-size: 13.5px; outline: none;
  }}
  .filter-input:focus {{ border-color: var(--border-focus); }}
  ul {{ list-style: none; }}
  .item {{
    display: flex; align-items: center; gap: 12px; padding: 12px 16px;
    background: var(--card); border: 1px solid var(--border);
    border-radius: 10px; margin-bottom: 8px;
  }}
  .bm-icon {{ color: var(--muted); flex-shrink: 0; }}
  .item .title {{
    font-weight: 600; font-size: 13.5px; color: var(--fg); text-decoration: none;
    flex: 0 0 auto; max-width: 260px; overflow: hidden;
    text-overflow: ellipsis; white-space: nowrap;
  }}
  .item .title:hover {{ text-decoration: underline; }}
  .item .url {{
    color: var(--muted); font-size: 12.5px; flex: 1 1 auto;
    overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
  }}
  .item .remove {{
    background: transparent; border: 1px solid transparent;
    color: var(--muted); border-radius: 6px; width: 28px; height: 28px;
    cursor: pointer; display: flex; align-items: center; justify-content: center;
  }}
  .item .remove:hover {{ border-color: var(--border-focus); color: var(--fg); }}
  .empty {{
    color: var(--muted); padding: 36px; text-align: center; font-size: 13.5px;
    background: var(--card); border: 1px solid var(--border);
    border-radius: 10px; list-style: none;
  }}
</style>
</head>
<body>
  <div class="container">
    <div class="top-bar">
      <div>
        <h1>{heading}</h1>
        <div class="count" id="bm-count">{count} {items_lbl}</div>
      </div>
    </div>

    <form id="add-bm-form" class="add-form">
      <input type="text" id="bm-title" placeholder="{title_ph}">
      <input type="text" id="bm-url" placeholder="https://example.com" required>
      <button type="submit">{add_btn}</button>
    </form>

    <input type="text" id="bm-filter" class="filter-input" placeholder="{search_ph}">

    <ul id="bm-list">{rows}</ul>
  </div>
  <script>
    {shim}
    (function () {{
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

      var addForm = document.getElementById('add-bm-form');
      if (addForm) {{
        addForm.addEventListener('submit', function (e) {{
          e.preventDefault();
          var title = document.getElementById('bm-title').value.trim();
          var url = document.getElementById('bm-url').value.trim();
          if (!url) return;
          if (window.servoGtk && window.servoGtk.messageHandlers &&
              window.servoGtk.messageHandlers.kestrel) {{
            window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify({{
              action: 'add-bookmark',
              title: title || url,
              url: url
            }}));
          }}
        }});
      }}

      var filterInput = document.getElementById('bm-filter');
      if (filterInput) {{
        filterInput.addEventListener('input', function () {{
          var q = filterInput.value.toLowerCase().trim();
          document.querySelectorAll('#bm-list .item').forEach(function (li) {{
            var hay = li.getAttribute('data-search') || '';
            li.style.display = (!q || hay.indexOf(q) !== -1) ? 'flex' : 'none';
          }});
        }});
      }}
    }})();
  </script>
</body>
</html>"#,
        lang = lang,
        theme_class = theme_class,
        heading = t("Bookmarks", "Trang đánh dấu"),
        count = records.len(),
        items_lbl = t("saved items", "trang đã lưu"),
        title_ph = t("Title (optional)", "Tiêu đề (tùy chọn)"),
        add_btn = t("Add Bookmark", "Thêm"),
        search_ph = t("Filter bookmarks...", "Tìm kiếm trong bookmark..."),
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
