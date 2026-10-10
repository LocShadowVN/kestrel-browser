//! Trang lịch sử nội bộ — kestrel://history

pub fn render(records: &[(String, String, Option<String>)]) -> String {
    let mut rows = String::new();
    for (url, title, ts) in records {
        let ts_display = ts.as_deref().unwrap_or("");
        rows.push_str(&format!(
            r#"<tr><td class="title">{}</td><td class="url"><a href="{}">{}</a></td><td class="time">{}</td></tr>"#,
            escape(title),
            escape(url),
            escape(url),
            escape(ts_display),
        ));
    }
    if rows.is_empty() {
        rows.push_str(r#"<tr><td colspan="3" class="empty">No history yet.</td></tr>"#);
    }
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Kestrel — History</title>
<style>
  :root {{ --bg:#fafafa; --fg:#1a1a1a; --accent:#0a84ff; --card:#fff; --border:#e5e5e5; --muted:#6b6b6b; }}
  @media (prefers-color-scheme: dark) {{
    :root {{ --bg:#1c1c1e; --fg:#f5f5f7; --card:#2c2c2e; --border:#3a3a3c; --muted:#8e8e93; }}
  }}
  * {{ box-sizing:border-box; margin:0; padding:0; }}
  body {{ font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;
          background:var(--bg); color:var(--fg); padding:48px 24px; }}
  .container {{ max-width:900px; margin:0 auto; }}
  h1 {{ font-size:28px; font-weight:700; margin-bottom:8px; }}
  .count {{ font-size:13px; color:var(--muted); margin-bottom:24px; }}
  table {{ width:100%; border-collapse:collapse; background:var(--card);
           border:1px solid var(--border); border-radius:12px; overflow:hidden; }}
  th {{ text-align:left; padding:12px 16px; font-size:12px; font-weight:600;
        text-transform:uppercase; letter-spacing:0.5px; color:var(--muted);
        border-bottom:1px solid var(--border); }}
  td {{ padding:12px 16px; font-size:14px; border-bottom:1px solid var(--border); }}
  tr:last-child td {{ border-bottom:none; }}
  .title {{ font-weight:600; }}
  .url a {{ color:var(--accent); text-decoration:none; }}
  .url a:hover {{ text-decoration:underline; }}
  .time {{ color:var(--muted); font-size:12px; white-space:nowrap; }}
  .empty {{ text-align:center; color:var(--muted); padding:32px; }}
</style>
</head>
<body>
  <div class="container">
    <h1>History</h1>
    <div class="count">{count} entries</div>
    <table>
      <thead><tr><th>Title</th><th>URL</th><th>Time</th></tr></thead>
      <tbody>{rows}</tbody>
    </table>
  </div>
</body>
</html>"#,
        count = records.len(),
        rows = rows
    )
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
