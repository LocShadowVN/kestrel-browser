//! Trang cài đặt nội bộ — kestrel://settings

use crate::internal_pages::NAV_SHIM;

pub fn render(
    lang: &str,
    homepage: &str,
    search_engine: &str,
    dark: bool,
) -> String {
    let vi = lang == "vi";
    let en_sel = if lang == "en" { " selected" } else { "" };
    let vi_sel = if vi { " selected" } else { "" };
    let dark_checked = if dark { " checked" } else { "" };

    let t = |en: &str, vn: &str| -> String {
        if vi { vn.to_string() } else { en.to_string() }
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="{lang}">
<head>
<meta charset="utf-8">
<title>Kestrel — {title}</title>
<style>
  :root {{ --bg:#fafafa; --fg:#1a1a1a; --accent:#0a84ff; --card:#fff; --border:#e5e5e5; --muted:#6b6b6b; }}
  @media (prefers-color-scheme: dark) {{
    :root {{ --bg:#1c1c1e; --fg:#f5f5f7; --card:#2c2c2e; --border:#3a3a3c; --muted:#8e8e93; }}
  }}
  * {{ box-sizing:border-box; margin:0; padding:0; }}
  body {{ font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;
          background:var(--bg); color:var(--fg); padding:48px 24px; }}
  .container {{ max-width:680px; margin:0 auto; }}
  h1 {{ font-size:28px; font-weight:700; margin-bottom:32px; letter-spacing:-0.5px; }}
  h2 {{ font-size:12px; font-weight:600; text-transform:uppercase;
        letter-spacing:1.2px; color:var(--muted); margin:32px 0 12px; }}
  .field {{ background:var(--card); border:1px solid var(--border);
            border-radius:12px; padding:16px 20px; margin-bottom:10px; }}
  label {{ display:block; font-size:14px; font-weight:600; margin-bottom:6px; }}
  .hint {{ font-size:12px; color:var(--muted); margin-bottom:10px; line-height:1.4; }}
  input[type=text], select {{ width:100%; padding:8px 12px; font-size:14px;
    border:1px solid var(--border); border-radius:8px;
    background:var(--bg); color:var(--fg); outline:none; }}
  input[type=text]:focus, select:focus {{ border-color:var(--accent); }}
  input[type=checkbox] {{ width:16px; height:16px; accent-color:var(--accent); }}
  .row {{ display:flex; align-items:center; gap:10px; }}
  .row label {{ margin:0; }}
  .actions {{ margin-top:24px; display:flex; gap:12px; }}
  button, a.btn {{ padding:10px 20px; font-size:14px; font-weight:600;
    border:none; border-radius:8px; cursor:pointer;
    text-decoration:none; display:inline-block; }}
  .primary {{ background:var(--accent); color:#fff; }}
  .secondary {{ background:var(--card); color:var(--fg); border:1px solid var(--border); }}
  .status {{ margin-top:16px; font-size:13px; color:var(--muted); min-height:18px; }}
  .about-row {{ display:flex; justify-content:space-between; align-items:center;
                padding:12px 20px; font-size:14px; }}
  .about-row + .about-row {{ border-top:1px solid var(--border); }}
  .about-card {{ background:var(--card); border:1px solid var(--border);
                 border-radius:12px; overflow:hidden; }}
  a {{ color:var(--accent); text-decoration:none; }}
  a:hover {{ text-decoration:underline; }}
  .disabled {{ opacity:0.5; pointer-events:none; }}
  .badge {{ font-size:11px; padding:2px 8px; border-radius:10px;
            background:var(--border); color:var(--muted); font-weight:600; }}
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
      <label for="search">{search_label}</label>
      <div class="hint">{search_hint}</div>
      <input type="text" id="search" name="search" value="{search_value}">
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
        <input type="hidden" name="dark" value="false">
        <input type="checkbox" id="dark" name="dark" value="true"{dark_checked}>
        <label for="dark">{dark_label}</label>
      </div>
      <div class="hint" style="margin-top:8px; margin-bottom:0;">{dark_hint}</div>
    </div>

    <h2>{privacy}</h2>
    <div class="field disabled">
      <div class="row">
        <input type="checkbox" disabled>
        <label>{block_ads} <span class="badge">Phase 2</span></label>
      </div>
      <div class="hint" style="margin-top:8px; margin-bottom:0;">{block_ads_hint}</div>
    </div>
    <div class="field disabled">
      <div class="row">
        <input type="checkbox" disabled>
        <label>{block_trackers} <span class="badge">Phase 2</span></label>
      </div>
      <div class="hint" style="margin-top:8px; margin-bottom:0;">{block_trackers_hint}</div>
    </div>
    <div class="field disabled">
      <div class="row">
        <input type="checkbox" disabled>
        <label>{block_cookies} <span class="badge">Phase 2</span></label>
      </div>
      <div class="hint" style="margin-top:8px; margin-bottom:0;">{block_cookies_hint}</div>
    </div>

    <div class="actions">
      <button type="submit" class="primary">{save}</button>
      <a href="kestrel://settings?reset=1" class="btn secondary">{reset}</a>
    </div>
    <div class="status" id="status"></div>
  </form>

  <h2>{about}</h2>
  <div class="about-card">
    <div class="about-row"><span>{version_label}</span><span class="badge">0.1.0-alpha</span></div>
    <div class="about-row"><span>{engine_label}</span><span>Servo</span></div>
    <div class="about-row"><span>{license_label}</span><span>GPL-3.0</span></div>
    <div class="about-row"><span>{source_label}</span><a href="https://github.com/LocShadowVN/kestrel-browser">GitHub</a></div>
  </div>
</div>

<script>
(function () {{
  var form = document.getElementById('settings-form');
  form.addEventListener('submit', function (e) {{
    e.preventDefault();
    var data = {{ action: 'save-settings' }};
    var els = form.elements;
    for (var i = 0; i < els.length; i++) {{
      var el = els[i];
      if (!el.name) continue;
      if (el.type === 'checkbox') data[el.name] = el.checked;
      else if (el.type === 'hidden' && el.name === 'dark') {{
        if (typeof data['dark'] === 'undefined') data['dark'] = el.value === 'true';
      }} else data[el.name] = el.value;
    }}
    if (window.servoGtk && window.servoGtk.messageHandlers &&
        window.servoGtk.messageHandlers.kestrel) {{
      window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify(data));
      document.getElementById('status').textContent = '{saved_msg}';
    }} else {{
      document.getElementById('status').textContent = '{no_channel_msg}';
      form.submit();
    }}
  }});
}})();
</script>
<script>{shim}</script>
</body>
</html>"#,
        lang = lang,
        title = t("Settings", "Cài đặt"),
        general = t("GENERAL", "CHUNG"),
        appearance = t("APPEARANCE", "GIAO DIỆN"),
        privacy = t("PRIVACY", "RIÊNG TƯ"),
        about = t("ABOUT", "GIỚI THIỆU"),
        homepage_label = t("Homepage", "Trang chủ"),
        homepage_hint = t("Page opened on Home button or at startup.", "Trang mở khi nhấn Home hoặc khởi động."),
        homepage_value = escape(homepage),
        search_label = t("Search engine", "Công cụ tìm kiếm"),
        search_hint = t("URL template; the query is appended after ?q=.", "URL template; từ khóa được nối sau ?q=."),
        search_value = escape(search_engine),
        language_label = t("Language", "Ngôn ngữ"),
        language_hint = t("Display language for internal pages.", "Ngôn ngữ hiển thị trang nội bộ."),
        dark_label = t("Dark mode", "Chế độ tối"),
        dark_hint = t("Prefer dark theme.", "Ưu tiên giao diện tối."),
        block_ads = t("Block ads", "Chặn quảng cáo"),
        block_ads_hint = t("EasyList network + cosmetic filtering. Requires servo-gtk fork.", "Lọc EasyList (network + cosmetic). Cần fork servo-gtk."),
        block_trackers = t("Block trackers", "Chặn tracker"),
        block_trackers_hint = t("EasyPrivacy filter list.", "Danh sách lọc EasyPrivacy."),
        block_cookies = t("Block third-party cookies", "Chặn cookie bên thứ ba"),
        block_cookies_hint = t("Reject cross-site cookies.", "Từ chối cookie cross-site."),
        save = t("Save", "Lưu"),
        reset = t("Reset", "Mặc định"),
        saved_msg = t("Saved.", "Đã lưu."),
        no_channel_msg = t("Native channel unavailable — trying form fallback.", "Kênh native không có — thử fallback form."),
        version_label = t("Version", "Phiên bản"),
        engine_label = t("Engine", "Engine"),
        license_label = t("License", "Giấy phép"),
        source_label = t("Source", "Mã nguồn"),
        en_sel = en_sel,
        vi_sel = vi_sel,
        dark_checked = dark_checked,
        shim = NAV_SHIM,
    );
    html
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
