//! Trang bookmark nội bộ — kestrel://bookmarks

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
  <div class="fav">★</div>
  <a class="title" href="{url}">{title}</a>
  <span class="url">{url}</span>
  <button class="remove" data-id="{id}" title="Remove">×</button>
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
                "No bookmarks yet. Click the star icon ★ in the address bar (or press Ctrl+D) to save a page.",
                "Chưa có trang đánh dấu nào. Nhấn biểu tượng ngôi sao ★ trên thanh địa chỉ (hoặc Ctrl+D) để lưu trang."
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
    --bg: #0c0e14; --fg: #f2f4fa; --accent: #ff622d;
    --card: #161924; --border: #262a3b; --muted: #8e94a8;
  }}
  html.light {{
    --bg: #f4f6fa; --fg: #151821; --accent: #ff622d;
    --card: #ffffff; --border: #dcdfe8; --muted: #5e6478;
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    background: var(--bg); color: var(--fg); padding: 48px 24px;
  }}
  .container {{ max-width: 840px; margin: 0 auto; }}
  .top-bar {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; flex-wrap: wrap; gap: 12px; }}
  h1 {{ font-size: 28px; font-weight: 800; }}
  .count {{ font-size: 13px; color: var(--muted); }}
  .add-form {{
    display: flex; gap: 10px; margin-bottom: 18px; flex-wrap: wrap;
    background: var(--card); border: 1px solid var(--border); padding: 14px; border-radius: 14px;
  }}
  .add-form input {{
    flex: 1 1 200px; padding: 9px 12px; border-radius: 8px;
    border: 1px solid var(--border); background: var(--bg); color: var(--fg); font-size: 13.5px; outline: none;
  }}
  .add-form input:focus {{ border-color: var(--accent); }}
  .add-form button {{
    padding: 9px 18px; border-radius: 8px; border: none;
    background: var(--accent); color: #fff; font-weight: 600; cursor: pointer;
  }}
  .filter-input {{
    width: 100%; padding: 10px 14px; margin-bottom: 16px; border-radius: 10px;
    border: 1px solid var(--border); background: var(--card); color: var(--fg); font-size: 14px; outline: none;
  }}
  .filter-input:focus {{ border-color: var(--accent); }}
  ul {{ list-style: none; }}
  .item {{
    display: flex; align-items: center; gap: 12px; padding: 14px 18px;
    background: var(--card); border: 1px solid var(--border);
    border-radius: 12px; margin-bottom: 8px;
  }}
  .fav {{ color: #ffb800; font-size: 16px; }}
  .item .title {{
    font-weight: 600; color: var(--fg); text-decoration: none;
    flex: 0 0 auto; max-width: 260px; overflow: hidden;
    text-overflow: ellipsis; white-space: nowrap;
  }}
  .item .title:hover {{ color: var(--accent); text-decoration: underline; }}
  .item .url {{
    color: var(--muted); font-size: 12.5px; flex: 1 1 auto;
    overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
  }}
  .item .remove {{
    background: transparent; border: 1px solid var(--border);
    color: var(--muted); border-radius: 7px; width: 28px; height: 28px;
    cursor: pointer; font-size: 16px; line-height: 1;
  }}
  .item .remove:hover {{ background: #e74c3c; color: #fff; border-color: #e74c3c; }}
  .empty {{
    color: var(--muted); padding: 36px; text-align: center;
    background: var(--card); border: 1px solid var(--border);
    border-radius: 12px; list-style: none;
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
      <button type="submit">+ {add_btn}</button>
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
