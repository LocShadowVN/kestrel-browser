//! Trang lịch sử nội bộ — kestrel://history
//!
//! HTML được tạo động: `render()` nhận danh sách bản ghi và sinh trang.

/// Sinh HTML cho trang lịch sử từ danh sách bản ghi.
pub fn render(records: &[(String, String, Option<String>)]) -> String {
    let mut rows = String::new();
    for (url, title, ts) in records {
        let ts_display = ts.as_deref().unwrap_or("");
        let title_esc = escape_html(title);
        let url_esc = escape_html(url);
        rows.push_str(&format!(
            r#"<tr>
  <td class="title">{title_esc}</td>
  <td class="url"><a href="{url_esc}">{url_esc}</a></td>
  <td class="time">{ts_display}</td>
</tr>"#
        ));
    }

    if rows.is_empty() {
        rows.push_str(r#"<tr><td colspan="3" class="empty">Chưa có lịch sử.</td></tr>"#);
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="vi">
<head>
<meta charset="utf-8">
<title>Kestrel History</title>
<style>
  :root {{ --bg: #fafafa; --fg: #1a1a1a; --card-bg: #fff; --card-border: #e5e5e5; --muted: #6b6b6b; --accent: #0a84ff; }}
  @media (prefers-color-scheme: dark) {{
    :root {{ --bg: #1c1c1e; --fg: #f5f5f7; --card-bg: #2c2c2e; --card-border: #3a3a3c; --muted: #8e8e93; }}
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: var(--bg); color: var(--fg); padding: 48px 24px; }}
  .container {{ max-width: 900px; margin: 0 auto; }}
  h1 {{ font-size: 28px; font-weight: 700; margin-bottom: 8px; letter-spacing: -0.5px; }}
  .count {{ font-size: 13px; color: var(--muted); margin-bottom: 24px; }}
  table {{ width: 100%; border-collapse: collapse; background: var(--card-bg); border: 1px solid var(--card-border); border-radius: 12px; overflow: hidden; }}
  th {{ text-align: left; padding: 12px 16px; font-size: 12px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; color: var(--muted); border-bottom: 1px solid var(--card-border); }}
  td {{ padding: 12px 16px; font-size: 14px; border-bottom: 1px solid var(--card-border); }}
  tr:last-child td {{ border-bottom: none; }}
  .title {{ font-weight: 600; }}
  .url a {{ color: var(--accent); text-decoration: none; }}
  .url a:hover {{ text-decoration: underline; }}
  .time {{ color: var(--muted); font-size: 12px; white-space: nowrap; }}
  .empty {{ text-align: center; color: var(--muted); padding: 32px; }}
</style>
</head>
<body>
  <div class="container">
    <h1>Lịch sử</h1>
    <div class="count">{count} bản ghi</div>
    <table>
      <thead><tr><th>Tiêu đề</th><th>URL</th><th>Thời gian</th></tr></thead>
      <tbody>{rows}</tbody>
    </table>
  </div>
</body>
</html>"#,
        count = records.len(),
        rows = rows
    )
}

/// Escape HTML để tránh XSS khi render title/URL từ database.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_html_handles_special_chars() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(escape_html("a & b"), "a &amp; b");
    }

    #[test]
    fn render_empty_shows_message() {
        let html = render(&[]);
        assert!(html.contains("Chưa có lịch sử"));
    }

    #[test]
    fn render_escapes_title() {
        let html = render(&[(
            "https://example.com".into(),
            "<script>alert(1)</script>".into(),
            None,
        )]);
        assert!(!html.contains("<script>alert"));
        assert!(html.contains("&lt;script&gt;"));
    }
}
