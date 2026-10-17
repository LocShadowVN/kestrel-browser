//! Trang lịch sử nội bộ — kestrel://history

use crate::internal_pages::NAV_SHIM;

pub fn render(
    records: &[(i64, String, String, Option<String>)],
    lang: &str,
    dark: bool,
) -> String {
    let vi = lang == "vi";
    let theme_class = if dark { "dark" } else { "light" };
    let t = |en: &str, vn: &str| -> String {
        if vi { vn.to_string() } else { en.to_string() }
    };

    let mut rows = String::new();
    for (id, url, title, ts) in records {
        let ts_display = ts.as_deref().unwrap_or("");
        rows.push_str(&format!(
            r#"<tr class="hist-row" data-search="{search}">
  <td class="title">{title}</td>
  <td class="url"><a href="{url}">{url}</a></td>
  <td class="time">{time}</td>
  <td class="act"><button class="del-row" data-id="{id}" title="Delete">×</button></td>
</tr>"#,
            id = id,
            title = escape(title),
            url = escape(url),
            time = escape(ts_display),
            search = escape(&format!("{} {}", title.to_lowercase(), url.to_lowercase())),
        ));
    }
    if rows.is_empty() {
        rows.push_str(&format!(
            r#"<tr><td colspan="4" class="empty">{}</td></tr>"#,
            t("No browsing history yet.", "Chưa có lịch sử duyệt web nào.")
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
  .container {{ max-width: 920px; margin: 0 auto; }}
  .header {{
    display: flex; justify-content: space-between; align-items: center;
    margin-bottom: 20px; flex-wrap: wrap; gap: 12px;
  }}
  h1 {{ font-size: 28px; font-weight: 800; }}
  .count {{ font-size: 13px; color: var(--muted); }}
  .clear-btn {{
    padding: 9px 16px; border-radius: 9px; font-size: 13px; font-weight: 600;
    background: rgba(255, 69, 58, 0.14); color: #ff5b52;
    border: 1px solid rgba(255, 69, 58, 0.35); cursor: pointer;
  }}
  .clear-btn:hover {{ background: #e74c3c; color: #fff; }}
  .filter-input {{
    width: 100%; padding: 10px 14px; margin-bottom: 16px; border-radius: 10px;
    border: 1px solid var(--border); background: var(--card); color: var(--fg); font-size: 14px; outline: none;
  }}
  .filter-input:focus {{ border-color: var(--accent); }}
  table {{
    width: 100%; border-collapse: collapse; background: var(--card);
    border: 1px solid var(--border); border-radius: 14px; overflow: hidden;
  }}
  th {{
    text-align: left; padding: 12px 16px; font-size: 11.5px; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.6px; color: var(--muted);
    border-bottom: 1px solid var(--border);
  }}
  td {{ padding: 12px 16px; font-size: 13.5px; border-bottom: 1px solid var(--border); }}
  tr:last-child td {{ border-bottom: none; }}
  .title {{ font-weight: 600; max-width: 240px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }}
  .url {{ max-width: 380px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }}
  .url a {{ color: var(--accent); text-decoration: none; }}
  .url a:hover {{ text-decoration: underline; }}
  .time {{ color: var(--muted); font-size: 12px; white-space: nowrap; }}
  .del-row {{
    background: transparent; border: 1px solid var(--border); color: var(--muted);
    border-radius: 6px; width: 26px; height: 26px; cursor: pointer;
  }}
  .del-row:hover {{ background: #e74c3c; color: #fff; border-color: #e74c3c; }}
  .empty {{ text-align: center; color: var(--muted); padding: 36px; }}
</style>
</head>
<body>
  <div class="container">
    <div class="header">
      <div>
        <h1>{heading}</h1>
        <div class="count">{count} {entries_lbl}</div>
      </div>
      <button id="clear-all" class="clear-btn">{clear_lbl}</button>
    </div>

    <input type="text" id="hist-filter" class="filter-input" placeholder="{filter_ph}">

    <table>
      <thead><tr><th>{col_title}</th><th>URL</th><th>{col_time}</th><th></th></tr></thead>
      <tbody id="hist-body">{rows}</tbody>
    </table>
  </div>
  <script>
    {shim}
    (function () {{
      var clearBtn = document.getElementById('clear-all');
      if (clearBtn) {{
        clearBtn.addEventListener('click', function () {{
          if (window.servoGtk && window.servoGtk.messageHandlers &&
              window.servoGtk.messageHandlers.kestrel) {{
            window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify({{
              action: 'clear-history'
            }}));
          }}
        }});
      }}

      document.addEventListener('click', function (e) {{
        var btn = e.target.closest('.del-row');
        if (!btn) return;
        var id = parseInt(btn.getAttribute('data-id'), 10);
        if (window.servoGtk && window.servoGtk.messageHandlers &&
            window.servoGtk.messageHandlers.kestrel) {{
          window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify({{
            action: 'delete-history',
            id: id
          }}));
          var tr = btn.closest('tr');
          if (tr) tr.remove();
        }}
      }});

      var filterInput = document.getElementById('hist-filter');
      if (filterInput) {{
        filterInput.addEventListener('input', function () {{
          var q = filterInput.value.toLowerCase().trim();
          document.querySelectorAll('#hist-body .hist-row').forEach(function (tr) {{
            var hay = tr.getAttribute('data-search') || '';
            tr.style.display = (!q || hay.indexOf(q) !== -1) ? '' : 'none';
          }});
        }});
      }}
    }})();
  </script>
</body>
</html>"#,
        lang = lang,
        theme_class = theme_class,
        heading = t("Browsing History", "Lịch sử duyệt web"),
        count = records.len(),
        entries_lbl = t("entries", "mục"),
        clear_lbl = t("Clear All History", "Xóa toàn bộ lịch sử"),
        filter_ph = t("Search history...", "Tìm trong lịch sử..."),
        col_title = t("Title", "Tiêu đề"),
        col_time = t("Visited", "Thời gian"),
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
