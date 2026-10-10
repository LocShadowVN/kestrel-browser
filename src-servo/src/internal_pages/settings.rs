//! Trang cài đặt nội bộ — kestrel://settings
//!
//! CSS đơn giản để Servo render nhanh, không dùng pseudo-element phức tạp.

/// Render trang settings với giá trị hiện tại.
pub fn render(lang: &str, homepage: &str, search_engine: &str, dark: bool) -> String {
    let en_selected = if lang == "en" { " selected" } else { "" };
    let vi_selected = if lang == "vi" { " selected" } else { "" };
    let dark_checked = if dark { " checked" } else { "" };

    format!(
        r#"<!DOCTYPE html>
<html lang="{lang}">
<head>
<meta charset="utf-8">
<title>Kestrel — Settings</title>
<style>
  :root {{ --bg:#fafafa; --fg:#1a1a1a; --accent:#0a84ff; --card:#fff; --border:#e5e5e5; --muted:#6b6b6b; }}
  @media (prefers-color-scheme: dark) {{
    :root {{ --bg:#1c1c1e; --fg:#f5f5f7; --card:#2c2c2e; --border:#3a3a3c; --muted:#8e8e93; }}
  }}
  * {{ box-sizing:border-box; margin:0; padding:0; }}
  body {{ font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;
          background:var(--bg); color:var(--fg); padding:48px 24px; }}
  .container {{ max-width:640px; margin:0 auto; }}
  h1 {{ font-size:28px; font-weight:700; margin-bottom:32px; }}
  h2 {{ font-size:12px; font-weight:600; text-transform:uppercase;
        letter-spacing:1px; color:var(--muted); margin:32px 0 12px; }}
  .field {{ background:var(--card); border:1px solid var(--border);
            border-radius:12px; padding:16px 20px; margin-bottom:10px; }}
  label {{ display:block; font-size:14px; font-weight:600; margin-bottom:6px; }}
  .hint {{ font-size:12px; color:var(--muted); margin-bottom:10px; line-height:1.4; }}
  input[type=text], select {{
    width:100%; padding:8px 12px; font-size:14px;
    border:1px solid var(--border); border-radius:8px;
    background:var(--bg); color:var(--fg); outline:none;
  }}
  input[type=text]:focus, select:focus {{ border-color:var(--accent); }}
  input[type=checkbox] {{ width:18px; height:18px; accent-color:var(--accent); }}
  .row {{ display:flex; align-items:center; gap:10px; }}
  .actions {{ margin-top:24px; display:flex; gap:12px; }}
  button {{ padding:10px 20px; font-size:14px; font-weight:600;
            border:none; border-radius:8px; cursor:pointer; }}
  .primary {{ background:var(--accent); color:#fff; }}
  .secondary {{ background:var(--card); color:var(--fg); border:1px solid var(--border); }}
  .status {{ margin-top:16px; font-size:13px; color:var(--muted); }}
</style>
</head>
<body>
  <div class="container">
    <h1>{title}</h1>

    <h2>{general}</h2>
    <div class="field">
      <label for="homepage">{homepage_label}</label>
      <div class="hint">{homepage_hint}</div>
      <input type="text" id="homepage" value="{homepage_value}">
    </div>
    <div class="field">
      <label for="search">{search_label}</label>
      <div class="hint">{search_hint}</div>
      <input type="text" id="search" value="{search_value}">
    </div>
    <div class="field">
      <label for="language">{language_label}</label>
      <div class="hint">{language_hint}</div>
      <select id="language">
        <option value="en"{en_sel}>English</option>
        <option value="vi"{vi_sel}>Tiếng Việt</option>
      </select>
    </div>

    <h2>{appearance}</h2>
    <div class="field">
      <div class="row">
        <input type="checkbox" id="dark"{dark_checked}>
        <label for="dark" style="margin:0;">{dark_label}</label>
      </div>
      <div class="hint" style="margin-top:6px; margin-bottom:0;">{dark_hint}</div>
    </div>

    <div class="actions">
      <button class="primary" id="save">{save}</button>
      <button class="secondary" id="reset">{reset}</button>
    </div>
    <div class="status" id="status"></div>
  </div>

  <script>
    document.getElementById('save').addEventListener('click', function () {{
      var payload = {{
        action: 'save-settings',
        homepage: document.getElementById('homepage').value,
        search_engine: document.getElementById('search').value,
        language: document.getElementById('language').value,
        dark_theme: document.getElementById('dark').checked
      }};
      if (window.servoGtk && window.servoGtk.messageHandlers && window.servoGtk.messageHandlers.kestrel) {{
        window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify(payload));
        document.getElementById('status').textContent = '{saved_msg}';
      }} else {{
        document.getElementById('status').textContent = '{no_channel_msg}';
      }}
    }});
    document.getElementById('reset').addEventListener('click', function () {{
      document.getElementById('homepage').value = 'kestrel://home';
      document.getElementById('search').value = 'https://search.brave.com/search?q=';
      document.getElementById('language').value = 'en';
      document.getElementById('dark').checked = false;
      document.getElementById('status').textContent = '{reset_msg}';
    }});
  </script>
</body>
</html>"#,
        lang = lang,
        title = if lang == "vi" { "Cài đặt" } else { "Settings" },
        general = if lang == "vi" { "Chung" } else { "General" },
        appearance = if lang == "vi" { "Giao diện" } else { "Appearance" },
        homepage_label = if lang == "vi" { "Trang chủ" } else { "Homepage" },
        homepage_hint = if lang == "vi" { "Trang mở khi nhấn nút Home hoặc khởi động." } else { "Page opened on Home button or at startup." },
        search_label = if lang == "vi" { "Công cụ tìm kiếm" } else { "Search engine" },
        search_hint = if lang == "vi" { "URL template, dùng ?q= cho từ khóa." } else { "URL template, uses ?q= for the query." },
        language_label = if lang == "vi" { "Ngôn ngữ" } else { "Language" },
        language_hint = if lang == "vi" { "Ngôn ngữ hiển thị trang nội bộ." } else { "Display language for internal pages." },
        dark_label = if lang == "vi" { "Chế độ tối" } else { "Dark mode" },
        dark_hint = if lang == "vi" { "Ưu tiên giao diện tối." } else { "Prefer dark theme." },
        save = if lang == "vi" { "Lưu" } else { "Save" },
        reset = if lang == "vi" { "Mặc định" } else { "Reset" },
        saved_msg = if lang == "vi" { "Đã lưu." } else { "Saved." },
        no_channel_msg = if lang == "vi" { "Kênh native chưa sẵn sàng." } else { "Native channel not ready." },
        reset_msg = if lang == "vi" { "Đã khôi phục (chưa lưu)." } else { "Reset (not saved)." },
        homepage_value = escape(homepage),
        search_value = escape(search_engine),
        en_sel = en_selected,
        vi_sel = vi_selected,
        dark_checked = dark_checked,
    )
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
