//! Trang cài đặt nội bộ — kestrel://settings

use crate::internal_pages::NAV_SHIM;

pub fn render(
    lang: &str,
    homepage: &str,
    search_engine: &str,
    dark: bool,
    adblock_level: u8,
) -> String {
    let vi = lang == "vi";
    let theme_class = if dark { "dark" } else { "light" };
    let en_sel = if lang == "en" { " selected" } else { "" };
    let vi_sel = if vi { " selected" } else { "" };
    let dark_checked = if dark { " checked" } else { "" };

    let ab_0 = if adblock_level == 0 { " selected" } else { "" };
    let ab_1 = if adblock_level == 1 { " selected" } else { "" };
    let ab_2 = if adblock_level >= 2 { " selected" } else { "" };

    let t = |en: &str, vn: &str| -> String {
        if vi { vn.to_string() } else { en.to_string() }
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="{lang}" class="{theme_class}">
<head>
<meta charset="utf-8">
<title>Kestrel — {title}</title>
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
  .container {{ max-width: 700px; margin: 0 auto; }}
  h1 {{ font-size: 28px; font-weight: 800; margin-bottom: 24px; letter-spacing: -0.5px; }}
  h2 {{
    font-size: 12px; font-weight: 700; text-transform: uppercase;
    letter-spacing: 1.2px; color: var(--accent); margin: 28px 0 12px;
  }}
  .field {{
    background: var(--card); border: 1px solid var(--border);
    border-radius: 14px; padding: 18px 20px; margin-bottom: 12px;
  }}
  label {{ display: block; font-size: 14px; font-weight: 600; margin-bottom: 6px; }}
  .hint {{ font-size: 12.5px; color: var(--muted); margin-bottom: 10px; line-height: 1.4; }}
  input[type=text], select {{
    width: 100%; padding: 10px 12px; font-size: 14px;
    border: 1px solid var(--border); border-radius: 9px;
    background: var(--bg); color: var(--fg); outline: none;
  }}
  input[type=text]:focus, select:focus {{ border-color: var(--accent); }}
  input[type=checkbox] {{ width: 18px; height: 18px; accent-color: var(--accent); }}
  .row {{ display: flex; align-items: center; gap: 10px; }}
  .row label {{ margin: 0; }}
  .presets {{ display: flex; gap: 8px; flex-wrap: wrap; margin-top: 8px; }}
  .preset-btn {{
    padding: 5px 10px; font-size: 12px; border-radius: 6px;
    background: var(--bg); color: var(--muted); border: 1px solid var(--border);
    cursor: pointer;
  }}
  .preset-btn:hover {{ border-color: var(--accent); color: var(--fg); }}
  .actions {{ margin-top: 24px; display: flex; gap: 12px; flex-wrap: wrap; }}
  button, a.btn {{
    padding: 10px 22px; font-size: 14px; font-weight: 600;
    border: none; border-radius: 10px; cursor: pointer;
    text-decoration: none; display: inline-block;
  }}
  .primary {{ background: var(--accent); color: #fff; }}
  .secondary {{ background: var(--card); color: var(--fg); border: 1px solid var(--border); }}
  .danger {{ background: rgba(255, 69, 58, 0.14); color: #ff5b52; border: 1px solid rgba(255, 69, 58, 0.35); }}
  .status {{ margin-top: 14px; font-size: 13.5px; color: #32d74b; font-weight: 600; min-height: 20px; }}
  .about-row {{
    display: flex; justify-content: space-between; align-items: center;
    padding: 13px 20px; font-size: 14px;
  }}
  .about-row + .about-row {{ border-top: 1px solid var(--border); }}
  .about-card {{
    background: var(--card); border: 1px solid var(--border);
    border-radius: 14px; overflow: hidden;
  }}
  a {{ color: var(--accent); text-decoration: none; }}
  .badge {{
    font-size: 11px; padding: 3px 9px; border-radius: 10px;
    background: rgba(255, 98, 45, 0.15); color: var(--accent); font-weight: 700;
  }}
</style>
</head>
<body>
<div class="container">
  <h1>{title}</h1>
  <form id="settings-form" action="kestrel://settings" method="get">
    <h2>{general}</h2>
    <div class="field">
      <label for="homepage">{homepage_label}</label>
      <div class="hint">{homepage_hint}</div>
      <input type="text" id="homepage" name="homepage" value="{homepage_value}">
    </div>
    <div class="field">
      <label for="search_engine">{search_label}</label>
      <div class="hint">{search_hint}</div>
      <input type="text" id="search_engine" name="search_engine" value="{search_value}">
      <div class="presets">
        <button type="button" class="preset-btn" data-url="https://search.brave.com/search?q=">Brave Search</button>
        <button type="button" class="preset-btn" data-url="https://duckduckgo.com/?q=">DuckDuckGo</button>
        <button type="button" class="preset-btn" data-url="https://www.google.com/search?q=">Google</button>
        <button type="button" class="preset-btn" data-url="https://www.bing.com/search?q=">Bing</button>
      </div>
    </div>
    <div class="field">
      <label for="language">{language_label}</label>
      <div class="hint">{language_hint}</div>
      <select id="language" name="language">
        <option value="en"{en_sel}>English</option>
        <option value="vi"{vi_sel}>Tiếng Việt</option>
      </select>
    </div>

    <h2>{appearance}</h2>
    <div class="field">
      <div class="row">
        <input type="checkbox" id="dark_theme" name="dark_theme" value="true"{dark_checked}>
        <label for="dark_theme">{dark_label}</label>
      </div>
      <div class="hint" style="margin-top:8px; margin-bottom:0;">{dark_hint}</div>
    </div>

    <h2>{privacy}</h2>
    <div class="field">
      <label for="adblock_level">{shields_label} <span class="badge">adblock-rust</span></label>
      <div class="hint">{shields_hint}</div>
      <select id="adblock_level" name="adblock_level">
        <option value="0"{ab_0}>{ab_off}</option>
        <option value="1"{ab_1}>{ab_std}</option>
        <option value="2"{ab_2}>{ab_agg}</option>
      </select>
    </div>
    <div class="field">
      <label>{clear_data_label}</label>
      <div class="hint">{clear_data_hint}</div>
      <div style="display:flex; gap:10px; margin-top:8px;">
        <button type="button" id="clear-history-btn" class="danger">{clear_history_btn}</button>
      </div>
    </div>

    <div class="actions">
      <button type="submit" class="primary">{save}</button>
      <button type="button" id="reset-btn" class="secondary">{reset}</button>
    </div>
    <div class="status" id="status"></div>
  </form>

  <h2>{about}</h2>
  <div class="about-card">
    <div class="about-row"><span>{version_label}</span><span class="badge">1.0.0-alpha</span></div>
    <div class="about-row"><span>{engine_label}</span><span>Servo + adblock-rust</span></div>
    <div class="about-row"><span>{license_label}</span><span>GPL-3.0</span></div>
    <div class="about-row"><span>{source_label}</span><a href="https://github.com/LocShadowVN/kestrel-browser">GitHub</a></div>
  </div>
</div>

<script>
(function () {{
  var form = document.getElementById('settings-form');
  var statusEl = document.getElementById('status');

  document.querySelectorAll('.preset-btn').forEach(function (btn) {{
    btn.addEventListener('click', function () {{
      document.getElementById('search_engine').value = btn.getAttribute('data-url');
    }});
  }});

  form.addEventListener('submit', function (e) {{
    e.preventDefault();
    var isDark = document.getElementById('dark_theme').checked;
    document.documentElement.className = isDark ? 'dark' : 'light';
    var data = {{
      action: 'save-settings',
      homepage: document.getElementById('homepage').value.trim(),
      search_engine: document.getElementById('search_engine').value.trim(),
      language: document.getElementById('language').value,
      dark_theme: isDark,
      adblock_level: parseInt(document.getElementById('adblock_level').value, 10)
    }};
    if (window.servoGtk && window.servoGtk.messageHandlers &&
        window.servoGtk.messageHandlers.kestrel) {{
      window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify(data));
      statusEl.textContent = '{saved_msg}';
    }} else {{
      form.submit();
    }}
  }});

  var resetBtn = document.getElementById('reset-btn');
  if (resetBtn) {{
    resetBtn.addEventListener('click', function () {{
      if (window.servoGtk && window.servoGtk.messageHandlers &&
          window.servoGtk.messageHandlers.kestrel) {{
        window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify({{
          action: 'reset-settings'
        }}));
      }}
    }});
  }}

  var clearHistBtn = document.getElementById('clear-history-btn');
  if (clearHistBtn) {{
    clearHistBtn.addEventListener('click', function () {{
      if (window.servoGtk && window.servoGtk.messageHandlers &&
          window.servoGtk.messageHandlers.kestrel) {{
        window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify({{
          action: 'clear-history'
        }}));
        statusEl.textContent = '{cleared_msg}';
      }}
    }});
  }}
}})();
</script>
<script>{shim}</script>
</body>
</html>"#,
        lang = lang,
        theme_class = theme_class,
        title = t("Settings", "Cài đặt"),
        general = t("GENERAL", "CÀI ĐẶT CHUNG"),
        appearance = t("APPEARANCE", "GIAO DIỆN"),
        privacy = t("KESTREL SHIELDS & PRIVACY", "KHIÊN BẢO VỆ & RIÊNG TƯ"),
        about = t("ABOUT KESTREL", "GIỚI THIỆU"),
        homepage_label = t("Homepage", "Trang chủ"),
        homepage_hint = t(
            "Page opened when clicking Home or opening a new tab.",
            "Trang mở khi nhấn nút Home hoặc mở tab mới."
        ),
        homepage_value = escape(homepage),
        search_label = t("Search Engine", "Công cụ tìm kiếm"),
        search_hint = t(
            "URL template; your search query is appended at the end.",
            "URL mẫu; từ khóa tìm kiếm sẽ được nối vào cuối."
        ),
        search_value = escape(search_engine),
        language_label = t("Language", "Ngôn ngữ"),
        language_hint = t(
            "Display language for Kestrel internal pages.",
            "Ngôn ngữ hiển thị cho các trang nội bộ của Kestrel."
        ),
        dark_label = t("Dark Theme", "Chế độ tối (Dark Mode)"),
        dark_hint = t(
            "Apply dark theme to browser toolbar, tabs, and internal pages.",
            "Áp dụng giao diện tối cho thanh công cụ, tab và các trang nội bộ."
        ),
        shields_label = t("Ad & Tracker Blocking Level", "Mức độ chặn quảng cáo & tracker"),
        shields_hint = t(
            "Powered by Brave's adblock-rust engine with EasyList, EasyPrivacy & Fanboy's Annoyances.",
            "Sử dụng engine adblock-rust của Brave với EasyList, EasyPrivacy & Fanboy's Annoyances."
        ),
        ab_off = t("Off — No filtering", "Tắt — Không chặn"),
        ab_std = t("Standard — EasyList + EasyPrivacy", "Tiêu chuẩn — EasyList + EasyPrivacy"),
        ab_agg = t(
            "Aggressive — Standard + Fanboy's Annoyances",
            "Mạnh — Tiêu chuẩn + Fanboy's Annoyances"
        ),
        clear_data_label = t("Browsing Data", "Dữ liệu duyệt web"),
        clear_data_hint = t(
            "Remove all recorded browsing history from the local SQLite database.",
            "Xóa toàn bộ lịch sử duyệt web khỏi cơ sở dữ liệu SQLite cục bộ."
        ),
        clear_history_btn = t("Clear Browsing History", "Xóa lịch sử duyệt web"),
        save = t("Save Changes", "Lưu thay đổi"),
        reset = t("Restore Defaults", "Khôi phục mặc định"),
        saved_msg = t("✓ Settings saved and applied.", "✓ Đã lưu và áp dụng cài đặt."),
        cleared_msg = t("✓ Browsing history cleared.", "✓ Đã xóa toàn bộ lịch sử duyệt web."),
        version_label = t("Version", "Phiên bản"),
        engine_label = t("Engine", "Nhân trình duyệt"),
        license_label = t("License", "Giấy phép"),
        source_label = t("Source Code", "Mã nguồn"),
        en_sel = en_sel,
        vi_sel = vi_sel,
        dark_checked = dark_checked,
        ab_0 = ab_0,
        ab_1 = ab_1,
        ab_2 = ab_2,
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
