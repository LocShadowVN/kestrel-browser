//! Trang chủ nội bộ — kestrel://home (Thiết kế đơn sắc, tối giản, không dùng emoji hay màu loè loẹt).

use crate::internal_pages::NAV_SHIM;

pub fn render(
    lang: &str,
    dark: bool,
    bookmark_count: usize,
    history_count: usize,
    adblock_level: u8,
) -> String {
    let vi = lang == "vi";
    let theme_class = if dark { "dark" } else { "light" };

    let t = |en: &str, vn: &str| -> String {
        if vi { vn.to_string() } else { en.to_string() }
    };

    let shield_status = match adblock_level {
        0 => t("Off", "Tắt"),
        2 => t("Aggressive", "Mạnh"),
        _ => t("Standard", "Tiêu chuẩn"),
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="{lang}" class="{theme_class}">
<head>
<meta charset="utf-8">
<title>Kestrel — {title}</title>
<style>
  :root {{
    --bg: #101116;
    --fg: #e6e8f0;
    --muted: #8a8f9e;
    --card: #171921;
    --card-hover: #1e202b;
    --border: #252834;
    --border-hover: #383c4e;
    --icon-bg: rgba(255, 255, 255, 0.04);
  }}
  html.light {{
    --bg: #f5f6f8;
    --fg: #16181f;
    --muted: #606575;
    --card: #ffffff;
    --card-hover: #f9fafc;
    --border: #dfe2ea;
    --border-hover: #b8becf;
    --icon-bg: rgba(0, 0, 0, 0.04);
  }}
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Inter, Roboto, sans-serif;
    background-color: var(--bg);
    color: var(--fg);
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 76px 24px 32px;
  }}
  .brand {{
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 8px;
    color: var(--fg);
  }}
  .logo {{
    font-size: 40px;
    font-weight: 700;
    letter-spacing: -1.2px;
  }}
  .tagline {{
    color: var(--muted);
    font-size: 13.5px;
    margin-bottom: 32px;
  }}
  .stats-bar {{
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
    justify-content: center;
    margin-bottom: 32px;
  }}
  .stat-pill {{
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 999px;
    padding: 7px 15px;
    font-size: 12.5px;
    color: var(--muted);
    display: flex;
    align-items: center;
    gap: 7px;
  }}
  .stat-pill svg {{
    color: var(--muted);
    flex-shrink: 0;
  }}
  .stat-pill strong {{
    color: var(--fg);
    font-weight: 600;
  }}
  .search-wrap {{
    width: 100%;
    max-width: 560px;
    position: relative;
    margin-bottom: 36px;
  }}
  .search-input {{
    width: 100%;
    padding: 14px 20px 14px 44px;
    font-size: 14.5px;
    border: 1px solid var(--border);
    border-radius: 24px;
    background: var(--card);
    color: var(--fg);
    outline: none;
    transition: border-color 0.15s ease, background 0.15s ease;
  }}
  .search-input:focus {{
    border-color: var(--border-hover);
    background: var(--card-hover);
  }}
  .search-icon {{
    position: absolute;
    left: 16px;
    top: 50%;
    transform: translateY(-50%);
    pointer-events: none;
    color: var(--muted);
  }}
  .cards {{
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: 12px;
    width: 100%;
    max-width: 700px;
  }}
  .card {{
    padding: 18px 16px;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 12px;
    text-decoration: none;
    color: var(--fg);
    transition: background 0.12s ease, border-color 0.12s ease;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }}
  .card:hover {{
    background: var(--card-hover);
    border-color: var(--border-hover);
  }}
  .card-icon {{
    width: 34px;
    height: 34px;
    border-radius: 8px;
    background: var(--icon-bg);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--fg);
    margin-bottom: 2px;
  }}
  .card-title {{ font-size: 14px; font-weight: 600; }}
  .card-desc {{ font-size: 12px; color: var(--muted); line-height: 1.4; }}
  .footer {{
    margin-top: auto;
    padding-top: 48px;
    font-size: 12px;
    color: var(--muted);
  }}
</style>
</head>
<body>
  <div class="brand">
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
      <polygon points="12 2 19 21 12 17 5 21 12 2"/>
    </svg>
    <div class="logo">Kestrel</div>
  </div>
  <div class="tagline">{tagline}</div>

  <div class="stats-bar">
    <div class="stat-pill">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
      </svg>
      <span>{shields_lbl}:</span>
      <strong>{shield_status}</strong>
    </div>
    <div class="stat-pill">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"/>
      </svg>
      <span>{bookmarks_lbl}:</span>
      <strong>{bookmark_count}</strong>
    </div>
    <div class="stat-pill">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>
      </svg>
      <span>{history_lbl}:</span>
      <strong>{history_count}</strong>
    </div>
  </div>

  <form id="home-search-form" class="search-wrap" action="https://search.brave.com/search" method="get">
    <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>
    </svg>
    <input id="home-search-input" class="search-input" type="text" name="q" placeholder="{search_placeholder}" autofocus>
  </form>

  <div class="cards">
    <a class="card" href="kestrel://bookmarks">
      <div class="card-icon">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"/></svg>
      </div>
      <div class="card-title">{bookmarks_card}</div>
      <div class="card-desc">{bookmarks_desc}</div>
    </a>
    <a class="card" href="kestrel://history">
      <div class="card-icon">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><path d="M12 7v5l4 2"/></svg>
      </div>
      <div class="card-title">{history_card}</div>
      <div class="card-desc">{history_desc}</div>
    </a>
    <a class="card" href="kestrel://settings">
      <div class="card-icon">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
      </div>
      <div class="card-title">{settings_card}</div>
      <div class="card-desc">{settings_desc}</div>
    </a>
    <a class="card" href="https://servo.org">
      <div class="card-icon">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
      </div>
      <div class="card-title">Servo Engine</div>
      <div class="card-desc">{servo_desc}</div>
    </a>
  </div>

  <div class="footer">Kestrel Browser · Rust · GTK4 · Servo</div>

  <script>
    {shim}
    (function () {{
      var form = document.getElementById('home-search-form');
      var input = document.getElementById('home-search-input');
      if (form && input) {{
        form.addEventListener('submit', function (e) {{
          var q = input.value.trim();
          if (!q) {{ e.preventDefault(); return; }}
          if (window.servoGtk && window.servoGtk.messageHandlers && window.servoGtk.messageHandlers.kestrel) {{
            e.preventDefault();
            window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify({{
              action: 'navigate',
              url: q
            }}));
          }}
        }});
      }}
    }})();
  </script>
</body>
</html>"#,
        lang = lang,
        theme_class = theme_class,
        title = t("New Tab", "Tab mới"),
        tagline = t(
            "Lightweight, pure-Rust web browser with built-in ad & tracker blocking",
            "Trình duyệt thuần Rust nhẹ, riêng tư, tích hợp sẵn chặn quảng cáo"
        ),
        shields_lbl = t("Shields", "Khiên chắn"),
        shield_status = shield_status,
        bookmarks_lbl = t("Bookmarks", "Đã lưu"),
        bookmark_count = bookmark_count,
        history_lbl = t("History", "Lịch sử"),
        history_count = history_count,
        search_placeholder = t(
            "Search the web or enter URL...",
            "Tìm kiếm hoặc nhập địa chỉ URL..."
        ),
        bookmarks_card = t("Bookmarks", "Trang đánh dấu"),
        bookmarks_desc = t("Saved pages", "Các trang đã lưu"),
        history_card = t("History", "Lịch sử"),
        history_desc = t("Visited pages", "Các trang đã truy cập"),
        settings_card = t("Settings", "Cài đặt"),
        settings_desc = t("Preferences & Shields", "Tùy chỉnh & chặn quảng cáo"),
        servo_desc = t("Browser engine project", "Dự án nhân trình duyệt"),
        shim = NAV_SHIM,
    )
}
