//! Trang cài đặt nội bộ — kestrel://settings.

use crate::internal_pages::NAV_SHIM;

#[allow(dead_code)]
pub fn render(
    lang: &str,
    homepage: &str,
    search_engine: &str,
    dark: bool,
    adblock_level: u8,
) -> String {
    render_full(
        lang,
        homepage,
        search_engine,
        dark,
        adblock_level,
        shared::DEFAULT_USER_AGENT,
    )
}

pub fn render_full(
    lang: &str,
    homepage: &str,
    search_engine: &str,
    dark: bool,
    adblock_level: u8,
    user_agent: &str,
) -> String {
    let vi = lang == "vi";
    let theme_class = if dark { "dark" } else { "light" };

    let en_active = if !vi { " active" } else { "" };
    let vi_active = if vi { " active" } else { "" };

    let dark_active = if dark { " active" } else { "" };
    let light_active = if !dark { " active" } else { "" };

    let ab_0_active = if adblock_level == 0 { " active" } else { "" };
    let ab_1_active = if adblock_level == 1 { " active" } else { "" };
    let ab_2_active = if adblock_level >= 2 { " active" } else { "" };

    let brave_active = if search_engine.contains("brave.com") { " active" } else { "" };
    let ddg_active = if search_engine.contains("duckduckgo.com") { " active" } else { "" };
    let google_active = if search_engine.contains("google.com") { " active" } else { "" };
    let bing_active = if search_engine.contains("bing.com") { " active" } else { "" };

    let ua_chrome_active = if user_agent.contains("Chrome/") { " active" } else { "" };
    let ua_ff_active = if user_agent.contains("Firefox/") && !user_agent.contains("Servo/") {
        " active"
    } else {
        ""
    };
    let ua_safari_active = if user_agent.contains("Version/") && user_agent.contains("Safari/") {
        " active"
    } else {
        ""
    };
    let ua_servo_active = if user_agent.contains("Servo/") { " active" } else { "" };

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
    --card: #171a22; --card-alt: #12151c; --border: #252936; --border-focus: #555c78;
    --btn-bg: #e6e8f0; --btn-fg: #111318;
  }}
  html.light {{
    --bg: #f4f5f8; --fg: #16181f; --muted: #606575;
    --card: #ffffff; --card-alt: #eef0f6; --border: #dcdfe8; --border-focus: #687089;
    --btn-bg: #16181f; --btn-fg: #ffffff;
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Inter, Roboto, sans-serif;
    background: var(--bg); color: var(--fg); padding: 44px 24px 64px;
  }}
  .container {{ max-width: 700px; margin: 0 auto; }}
  .top-bar {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }}
  h1 {{ font-size: 25px; font-weight: 700; letter-spacing: -0.4px; }}
  .back-link {{
    font-size: 13px; color: var(--muted); text-decoration: none;
    padding: 6px 12px; border: 1px solid var(--border); border-radius: 8px; background: var(--card);
  }}
  .back-link:hover {{ color: var(--fg); border-color: var(--border-focus); }}
  h2 {{
    font-size: 11.5px; font-weight: 600; text-transform: uppercase;
    letter-spacing: 1px; color: var(--muted); margin: 26px 0 10px;
  }}
  .field {{
    background: var(--card); border: 1px solid var(--border);
    border-radius: 12px; padding: 16px 18px; margin-bottom: 10px;
  }}
  .field-header {{
    display: flex; justify-content: space-between; align-items: center;
    gap: 12px; flex-wrap: wrap;
  }}
  label {{ display: block; font-size: 13.5px; font-weight: 600; margin-bottom: 3px; }}
  .hint {{ font-size: 12px; color: var(--muted); margin-bottom: 10px; line-height: 1.4; }}
  input[type=text] {{
    width: 100%; padding: 9px 12px; font-size: 13.5px;
    border: 1px solid var(--border); border-radius: 8px;
    background: var(--bg); color: var(--fg); outline: none;
  }}
  input[type=text]:focus {{ border-color: var(--border-focus); }}
  .seg-group {{ display: flex; gap: 6px; flex-wrap: wrap; }}
  .seg-btn {{
    padding: 8px 14px; font-size: 12.5px; font-weight: 600;
    border-radius: 8px; border: 1px solid var(--border);
    background: var(--card-alt); color: var(--muted); cursor: pointer;
  }}
  .seg-btn:hover {{ border-color: var(--border-focus); color: var(--fg); }}
  .seg-btn.active {{
    background: var(--btn-bg); color: var(--btn-fg); border-color: var(--btn-bg);
  }}
  .shield-cards {{ display: flex; gap: 8px; flex-wrap: wrap; margin-top: 6px; }}
  .shield-card {{
    flex: 1 1 180px; text-align: left; padding: 12px 14px;
    border-radius: 10px; border: 1px solid var(--border);
    background: var(--card-alt); color: var(--fg); cursor: pointer;
  }}
  .shield-card:hover {{ border-color: var(--border-focus); }}
  .shield-card.active {{
    background: var(--btn-bg); color: var(--btn-fg); border-color: var(--btn-bg);
  }}
  .shield-card-title {{ font-size: 13px; font-weight: 700; margin-bottom: 3px; }}
  .shield-card-desc {{ font-size: 11.5px; opacity: 0.78; }}
  .presets {{ display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 10px; }}
  .actions {{ margin-top: 20px; display: flex; gap: 10px; flex-wrap: wrap; align-items: center; }}
  .btn-primary {{
    padding: 9px 18px; font-size: 13.5px; font-weight: 600;
    border: 1px solid var(--btn-bg); border-radius: 8px; cursor: pointer;
    background: var(--btn-bg); color: var(--btn-fg);
  }}
  .btn-secondary {{
    padding: 9px 16px; font-size: 13px; font-weight: 600;
    border: 1px solid var(--border); border-radius: 8px; cursor: pointer;
    background: var(--card); color: var(--fg);
  }}
  .btn-secondary:hover {{ border-color: var(--border-focus); }}
  .status {{ font-size: 13px; font-weight: 600; color: var(--fg); min-height: 18px; }}
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
    <h2>{appearance}</h2>
    <div class="field">
      <div class="field-header">
        <div>
          <label>{theme_label}</label>
          <div class="hint" style="margin-bottom:0;">{dark_hint}</div>
        </div>
        <div class="seg-group" id="theme-group">
          <button type="button" class="seg-btn{dark_active}" data-theme="dark">{dark_btn}</button>
          <button type="button" class="seg-btn{light_active}" data-theme="light">{light_btn}</button>
        </div>
      </div>
    </div>

    <div class="field">
      <div class="field-header">
        <div>
          <label>{language_label}</label>
          <div class="hint" style="margin-bottom:0;">{language_hint}</div>
        </div>
        <div class="seg-group" id="lang-group">
          <button type="button" class="seg-btn{en_active}" data-lang="en">English</button>
          <button type="button" class="seg-btn{vi_active}" data-lang="vi">Tiếng Việt</button>
        </div>
      </div>
    </div>

    <h2>{privacy}</h2>
    <div class="field">
      <label>{shields_label} <span class="badge">adblock-rust + Anti-Adblock</span></label>
      <div class="hint">{shields_hint}</div>
      <div class="shield-cards" id="shields-group">
        <button type="button" class="shield-card{ab_0_active}" data-level="0">
          <div class="shield-card-title">{ab_off_title}</div>
          <div class="shield-card-desc">{ab_off_desc}</div>
        </button>
        <button type="button" class="shield-card{ab_1_active}" data-level="1">
          <div class="shield-card-title">{ab_std_title}</div>
          <div class="shield-card-desc">EasyList + EasyPrivacy</div>
        </button>
        <button type="button" class="shield-card{ab_2_active}" data-level="2">
          <div class="shield-card-title">{ab_agg_title}</div>
          <div class="shield-card-desc">Standard + Fanboy's Annoyances</div>
        </button>
      </div>
    </div>

    <div class="field">
      <label for="user_agent">{ua_label} <span class="badge">{ua_badge}</span></label>
      <div class="hint">{ua_hint}</div>
      <div class="presets" id="ua-group">
        <button type="button" class="seg-btn{ua_chrome_active}" data-ua="Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36">Chrome (Default)</button>
        <button type="button" class="seg-btn{ua_ff_active}" data-ua="Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0">Firefox</button>
        <button type="button" class="seg-btn{ua_safari_active}" data-ua="Mozilla/5.0 (Macintosh; Intel Mac OS X 14_7_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.1 Safari/605.1.15">Safari</button>
        <button type="button" class="seg-btn{ua_servo_active}" data-ua="Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Servo/1.0 Firefox/128.0">Servo Native</button>
      </div>
      <input type="text" id="user_agent" name="user_agent" value="{ua_value}">
    </div>

    <h2>{general}</h2>
    <div class="field">
      <label for="search_engine">{search_label}</label>
      <div class="hint">{search_hint}</div>
      <div class="presets" id="preset-group">
        <button type="button" class="seg-btn{brave_active}" data-url="https://search.brave.com/search?q=">Brave Search</button>
        <button type="button" class="seg-btn{ddg_active}" data-url="https://duckduckgo.com/?q=">DuckDuckGo</button>
        <button type="button" class="seg-btn{google_active}" data-url="https://www.google.com/search?q=">Google</button>
        <button type="button" class="seg-btn{bing_active}" data-url="https://www.bing.com/search?q=">Bing</button>
      </div>
      <input type="text" id="search_engine" name="search_engine" value="{search_value}">
    </div>

    <div class="field">
      <label for="homepage">{homepage_label}</label>
      <div class="hint">{homepage_hint}</div>
      <input type="text" id="homepage" name="homepage" value="{homepage_value}">
    </div>

    <div class="field">
      <div class="field-header">
        <div>
          <label>{clear_data_label}</label>
          <div class="hint" style="margin-bottom:0;">{clear_data_hint}</div>
        </div>
        <button type="button" id="clear-history-btn" class="btn-secondary">{clear_history_btn}</button>
      </div>
    </div>

    <div class="actions">
      <button type="submit" class="btn-primary">{save}</button>
      <button type="button" id="reset-btn" class="btn-secondary">{reset}</button>
      <span class="status" id="status"></span>
    </div>
  </form>

  <h2 id="about">{about}</h2>
  <div class="about-card">
    <div class="about-row"><span>{version_label}</span><span class="badge">1.0.0-alpha</span></div>
    <div class="about-row"><span>{engine_label}</span><span>Servo 0.6 + adblock-rust</span></div>
    <div class="about-row"><span>{license_label}</span><span>GPL-3.0</span></div>
    <div class="about-row"><span>{source_label}</span><a href="https://github.com/LocShadowVN/kestrel-browser">GitHub</a></div>
  </div>
</div>

<script>{shim}</script>
<script>
(function () {{
  var state = {{
    language: '{lang}',
    dark_theme: {dark_js},
    adblock_level: {adblock_level}
  }};

  var form = document.getElementById('settings-form');
  var statusEl = document.getElementById('status');
  var searchInput = document.getElementById('search_engine');
  var homeInput = document.getElementById('homepage');
  var uaInput = document.getElementById('user_agent');

  function showStatus(msg) {{
    if (!statusEl) return;
    statusEl.textContent = '✓ ' + msg;
  }}

  function pushSettings() {{
    var payload = {{
      action: 'save-settings',
      homepage: (homeInput && homeInput.value.trim()) || 'kestrel://home',
      search_engine: (searchInput && searchInput.value.trim()) || 'https://search.brave.com/search?q=',
      user_agent: (uaInput && uaInput.value.trim()) || '',
      language: state.language,
      dark_theme: state.dark_theme,
      adblock_level: state.adblock_level
    }};
    if (window.__kestrelSend) {{
      window.__kestrelSend(payload);
      showStatus('{saved_msg}');
    }}
  }}

  document.querySelectorAll('#theme-group .seg-btn').forEach(function (btn) {{
    btn.addEventListener('click', function () {{
      document.querySelectorAll('#theme-group .seg-btn').forEach(function (b) {{ b.classList.remove('active'); }});
      btn.classList.add('active');
      state.dark_theme = btn.getAttribute('data-theme') === 'dark';
      document.documentElement.className = state.dark_theme ? 'dark' : 'light';
      pushSettings();
    }});
  }});

  document.querySelectorAll('#lang-group .seg-btn').forEach(function (btn) {{
    btn.addEventListener('click', function () {{
      document.querySelectorAll('#lang-group .seg-btn').forEach(function (b) {{ b.classList.remove('active'); }});
      btn.classList.add('active');
      state.language = btn.getAttribute('data-lang') || 'en';
      pushSettings();
    }});
  }});

  document.querySelectorAll('#shields-group .shield-card').forEach(function (card) {{
    card.addEventListener('click', function () {{
      document.querySelectorAll('#shields-group .shield-card').forEach(function (c) {{ c.classList.remove('active'); }});
      card.classList.add('active');
      state.adblock_level = parseInt(card.getAttribute('data-level') || '1', 10);
      pushSettings();
    }});
  }});

  document.querySelectorAll('#ua-group .seg-btn').forEach(function (btn) {{
    btn.addEventListener('click', function () {{
      document.querySelectorAll('#ua-group .seg-btn').forEach(function (b) {{ b.classList.remove('active'); }});
      btn.classList.add('active');
      if (uaInput) uaInput.value = btn.getAttribute('data-ua') || '';
      pushSettings();
    }});
  }});

  document.querySelectorAll('#preset-group .seg-btn').forEach(function (btn) {{
    btn.addEventListener('click', function () {{
      document.querySelectorAll('#preset-group .seg-btn').forEach(function (b) {{ b.classList.remove('active'); }});
      btn.classList.add('active');
      if (searchInput) searchInput.value = btn.getAttribute('data-url') || '';
      pushSettings();
    }});
  }});

  if (form) {{
    form.addEventListener('submit', function (e) {{
      e.preventDefault();
      pushSettings();
    }});
  }}

  var resetBtn = document.getElementById('reset-btn');
  if (resetBtn) {{
    resetBtn.addEventListener('click', function () {{
      if (window.__kestrelSend) {{
        window.__kestrelSend({{ action: 'reset-settings' }});
      }}
    }});
  }}

  var clearHistBtn = document.getElementById('clear-history-btn');
  if (clearHistBtn) {{
    clearHistBtn.addEventListener('click', function () {{
      if (window.__kestrelSend) {{
        window.__kestrelSend({{ action: 'clear-history' }});
        showStatus('{cleared_msg}');
      }}
    }});
  }}
}})();
</script>
</body>
</html>"#,
        lang = lang,
        theme_class = theme_class,
        dark_js = if dark { "true" } else { "false" },
        adblock_level = adblock_level,
        title = t("Settings", "Cài đặt"),
        home_lbl = t("Home", "Trang chủ"),
        general = t("SEARCH & STARTUP", "TÌM KIẾM & KHỞI ĐỘNG"),
        appearance = t("APPEARANCE & LANGUAGE", "GIAO DIỆN & NGÔN NGỮ"),
        privacy = t("KESTREL SHIELDS & COMPATIBILITY", "KHIÊN CHẮN & TƯƠNG THÍCH WEB"),
        about = t("ABOUT KESTREL", "GIỚI THIỆU"),
        theme_label = t("Browser Theme", "Giao diện trình duyệt"),
        dark_btn = t("Dark", "Tối (Dark)"),
        light_btn = t("Light", "Sáng (Light)"),
        dark_hint = t(
            "Switch instantly between dark and light themes.",
            "Chuyển đổi tức thì giữa chế độ Tối và Sáng."
        ),
        language_label = t("Display Language", "Ngôn ngữ hiển thị"),
        language_hint = t(
            "Language for Kestrel internal pages and dashboard.",
            "Ngôn ngữ cho các trang nội bộ của Kestrel."
        ),
        shields_label = t("Ad & Tracker Blocking Level", "Mức độ chặn quảng cáo & tracker"),
        shields_hint = t(
            "Click to switch protection mode immediately across all tabs.",
            "Nhấn để chuyển đổi chế độ bảo vệ ngay lập tức trên mọi tab."
        ),
        ab_off_title = t("Off (Shields Down)", "Tắt (Shields Off)"),
        ab_off_desc = t("No filtering", "Không chặn mạng"),
        ab_std_title = t("Standard", "Tiêu chuẩn"),
        ab_agg_title = t("Aggressive", "Mạnh (Khuyên dùng)"),
        ua_label = t("User-Agent Spoofing", "Giả lập định danh trình duyệt (User-Agent)"),
        ua_badge = t("Web Compatibility", "Tương thích Web"),
        ua_hint = t(
            "Defaults to Desktop Chrome so websites serve full modern layouts (applies to new tabs).",
            "Mặc định giả lập Chrome để các trang web trả về giao diện đầy đủ (áp dụng cho tab mới)."
        ),
        ua_value = escape(user_agent),
        homepage_label = t("Homepage URL", "Địa chỉ trang chủ"),
        homepage_hint = t(
            "Opened when clicking Home or opening a new tab (Ctrl+T).",
            "Trang mở khi nhấn nút Home hoặc mở tab mới (Ctrl+T)."
        ),
        homepage_value = escape(homepage),
        search_label = t("Search Engine", "Công cụ tìm kiếm"),
        search_hint = t(
            "Click a preset below or enter a custom query URL.",
            "Chọn nhanh bên dưới hoặc nhập URL tìm kiếm tùy chỉnh."
        ),
        search_value = escape(search_engine),
        clear_data_label = t("Browsing History", "Lịch sử duyệt web"),
        clear_data_hint = t(
            "Delete all recorded browsing history from local SQLite storage.",
            "Xóa toàn bộ lịch sử duyệt web khỏi bộ nhớ SQLite cục bộ."
        ),
        clear_history_btn = t("Clear History", "Xóa lịch sử"),
        save = t("Save Changes", "Lưu thay đổi"),
        reset = t("Restore Defaults", "Khôi phục mặc định"),
        saved_msg = t("Settings applied.", "Đã áp dụng cài đặt."),
        cleared_msg = t("Browsing history cleared.", "Đã xóa lịch sử duyệt web."),
        version_label = t("Version", "Phiên bản"),
        engine_label = t("Engine", "Nhân trình duyệt"),
        license_label = t("License", "Giấy phép"),
        source_label = t("Source Code", "Mã nguồn"),
        en_active = en_active,
        vi_active = vi_active,
        dark_active = dark_active,
        light_active = light_active,
        ab_0_active = ab_0_active,
        ab_1_active = ab_1_active,
        ab_2_active = ab_2_active,
        ua_chrome_active = ua_chrome_active,
        ua_ff_active = ua_ff_active,
        ua_safari_active = ua_safari_active,
        ua_servo_active = ua_servo_active,
        brave_active = brave_active,
        ddg_active = ddg_active,
        google_active = google_active,
        bing_active = bing_active,
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
