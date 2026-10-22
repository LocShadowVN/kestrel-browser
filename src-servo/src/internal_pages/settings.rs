//! Trang cài đặt nội bộ — kestrel://settings & kestrel://about.

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
    let en_sel = if lang != "vi" { " selected" } else { "" };
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
    --bg: #111318; --fg: #e6e8f0; --muted: #8a8f9e;
    --card: #171a22; --border: #252936; --border-focus: #4a5068;
    --btn-bg: #e6e8f0; --btn-fg: #111318;
  }}
  html.light {{
    --bg: #f4f5f8; --fg: #16181f; --muted: #606575;
    --card: #ffffff; --border: #dcdfe8; --border-focus: #7a8096;
    --btn-bg: #16181f; --btn-fg: #ffffff;
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Inter, Roboto, sans-serif;
    background: var(--bg); color: var(--fg); padding: 44px 24px;
  }}
  .container {{ max-width: 680px; margin: 0 auto; }}
  .top-bar {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }}
  h1 {{ font-size: 25px; font-weight: 700; letter-spacing: -0.4px; }}
  .back-link {{
    font-size: 13px; color: var(--muted); text-decoration: none;
    padding: 6px 12px; border: 1px solid var(--border); border-radius: 8px; background: var(--card);
  }}
  .back-link:hover {{ color: var(--fg); border-color: var(--border-focus); }}
  h2 {{
    font-size: 11.5px; font-weight: 600; text-transform: uppercase;
    letter-spacing: 1px; color: var(--muted); margin: 28px 0 10px;
  }}
  .field {{
    background: var(--card); border: 1px solid var(--border);
    border-radius: 12px; padding: 16px 18px; margin-bottom: 10px;
  }}
  label {{ display: block; font-size: 13.5px; font-weight: 600; margin-bottom: 4px; }}
  .hint {{ font-size: 12px; color: var(--muted); margin-bottom: 10px; line-height: 1.4; }}
  input[type=text], select {{
    width: 100%; padding: 9px 12px; font-size: 13.5px;
    border: 1px solid var(--border); border-radius: 8px;
    background: var(--bg); color: var(--fg); outline: none;
  }}
  input[type=text]:focus, select:focus {{ border-color: var(--border-focus); }}
  input[type=checkbox] {{ width: 16px; height: 16px; accent-color: var(--fg); }}
  .row {{ display: flex; align-items: center; gap: 10px; }}
  .row label {{ margin: 0; }}
  .presets {{ display: flex; gap: 6px; flex-wrap: wrap; margin-top: 8px; }}
  .preset-btn {{
    padding: 5px 10px; font-size: 12px; border-radius: 6px;
    background: var(--bg); color: var(--muted); border: 1px solid var(--border);
    cursor: pointer;
  }}
  .preset-btn:hover {{ border-color: var(--border-focus); color: var(--fg); }}
  .actions {{ margin-top: 22px; display: flex; gap: 10px; flex-wrap: wrap; align-items: center; }}
  button {{
    padding: 9px 18px; font-size: 13.5px; font-weight: 600;
    border: 1px solid var(--border); border-radius: 8px; cursor: pointer;
  }}
  .primary {{ background: var(--btn-bg); color: var(--btn-fg); border-color: var(--btn-bg); }}
  .secondary {{ background: var(--card); color: var(--fg); }}
  .secondary:hover {{ border-color: var(--border-focus); }}
  .status {{ font-size: 13px; color: var(--muted); min-height: 18px; }}
  .about-row {{
    display: flex; justify-content: space-between; align-items: center;
    padding: 12px 18px; font-size: 13.5px;
  }}
  .about-row + .about-row {{ border-top: 1px solid var(--border); }}
  .about-card {{
    background: var(--card); border: 1px solid var(--border);
    border-radius: 12px; overflow: hidden;
  }}
  a {{ color: var(--fg); text-decoration: underline; }}
  .badge {{
    font-size: 11px; padding: 2px 8px; border-radius: 8px;
    background: var(--bg); border: 1px solid var(--border); color: var(--muted); font-weight: 500;
  }}
</style>
</head>
<body>
<div class="container">
  <div class="top-bar">
    <h1>{title}</h1>
    <a class="back-link" href="kestrel://home">← {home_lbl}</a>
  </div>
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
        <button type="button" id="clear-history-btn" class="secondary">{clear_history_btn}</button>
      </div>
    </div>

    <div class="actions">
      <button type="submit" class="primary">{save}</button>
      <button type="button" id="reset-btn" class="secondary">{reset}</button>
      <span class="status" id="status"></span>
    </div>
  </form>

  <h2 id="about">{about}</h2>
  <div class="about-card">
    <div class="about-row"><span>{version_label}</span><span class="badge">1.0.0-alpha</span></div>
    <div class="about-row"><span>{engine_label}</span><span>Servo + adblock-rust</span></div>
    <div class="about-row"><span>{license_label}</span><span>GPL-3.0</span></div>
    <div class="about-row"><span>{source_label}</span><a href="https://github.com/LocShadowVN/kestrel-browser">GitHub</a></div>
  </div>
</div>

<script>{shim}</script>
<script>
(function () {{
  var initialLang = '{lang}';
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
    var selectedLang = document.getElementById('language').value;
    document.documentElement.className = isDark ? 'dark' : 'light';
    var data = {{
      action: 'save-settings',
      homepage: document.getElementById('homepage').value.trim() || 'kestrel://home',
      search_engine: document.getElementById('search_engine').value.trim() || 'https://search.brave.com/search?q=',
      language: selectedLang,
      dark_theme: isDark,
      adblock_level: parseInt(document.getElementById('adblock_level').value, 10)
    }};
    if (window.__kestrelSend && window.__kestrelSend(data)) {{
      statusEl.textContent = '{saved_msg}';
      if (selectedLang !== initialLang) {{
        setTimeout(function () {{
          window.__kestrelSend({{ action: 'navigate', url: 'kestrel://settings' }});
        }}, 80);
      }}
    }} else {{
      form.submit();
    }}
  }});

  var resetBtn = document.getElementById('reset-btn');
  if (resetBtn) {{
    resetBtn.addEventListener('click', function () {{
      document.getElementById('homepage').value = 'kestrel://home';
      document.getElementById('search_engine').value = 'https://search.brave.com/search?q=';
      document.getElementById('language').value = 'en';
      document.getElementById('dark_theme').checked = true;
      document.getElementById('adblock_level').value = '1';
      document.documentElement.className = 'dark';
      if (window.__kestrelSend) {{
        window.__kestrelSend({{ action: 'reset-settings' }});
        statusEl.textContent = '{saved_msg}';
      }}
    }});
  }}

  var clearHistBtn = document.getElementById('clear-history-btn');
  if (clearHistBtn) {{
    clearHistBtn.addEventListener('click', function () {{
      if (window.__kestrelSend) {{
        window.__kestrelSend({{ action: 'clear-history' }});
        statusEl.textContent = '{cleared_msg}';
      }}
    }});
  }}
}})();
</script>
</body>
</html>"#,
        lang = lang,
        theme_class = theme_class,
        title = t("Settings", "Cài đặt"),
        home_lbl = t("Home", "Trang chủ"),
        general = t("GENERAL", "CÀI ĐẶT CHUNG"),
        appearance = t("APPEARANCE", "GIAO DIỆN"),
        privacy = t("SHIELDS & PRIVACY", "KHIÊN CHẮN & RIÊNG TƯ"),
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
        saved_msg = t("Settings saved.", "Đã lưu cài đặt."),
        cleared_msg = t("Browsing history cleared.", "Đã xóa lịch sử duyệt web."),
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
