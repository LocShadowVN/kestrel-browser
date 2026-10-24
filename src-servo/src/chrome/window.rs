//! MainWindow — layout: tab strip → toolbar → content.
//! - Không bao giờ gửi `adblock_blob` khi đang hiển thị trang nội bộ `kestrel://`
//!   giúp mở lại trình duyệt và mở tab mới (`+` / `Ctrl+T`) luôn hiện giao diện tức thì.
//! - Giải mã an toàn cả JSON đơn lẫn JSON bị double-encode từ `script-message-received`.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, EventControllerKey, Orientation,
    PropagationPhase,
};

use servo_gtk::user_content::{UserContentManager, UserScript};
use servo_gtk::{LoadEvent, WebView};

use crate::chrome::tabbar::TabBar;
use crate::chrome::toolbar::Toolbar;
use crate::engine::navigation;
use crate::internal_pages::{self, Page, NAV_SHIM};
use crate::storage::database::Database;
use crate::util::config::Config;

fn render_about_html(lang: &str, dark: bool) -> String {
    let theme_class = if dark { "dark" } else { "light" };
    let vi = lang == "vi";
    let home_lbl = if vi { "Trang chủ" } else { "Home" };
    let settings_lbl = if vi { "Cài đặt" } else { "Settings" };

    format!(
        r#"<!DOCTYPE html>
<html lang="{lang}" class="{theme_class}">
<head><meta charset="utf-8"><title>About Kestrel</title>
<style>
  :root {{
    --bg:#111318; --fg:#e6e8f0; --muted:#8a8f9e;
    --card:#171a22; --border:#252936; --border-hover:#4a5068;
  }}
  html.light {{
    --bg:#f4f5f8; --fg:#16181f; --muted:#606575;
    --card:#ffffff; --border:#dcdfe8; --border-hover:#7a8096;
  }}
  * {{ box-sizing:border-box; margin:0; padding:0; }}
  body {{
    font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Inter,Roboto,sans-serif;
    background:var(--bg); color:var(--fg); padding:52px 24px;
    max-width:660px; margin:0 auto; line-height:1.65;
  }}
  .card {{
    background:var(--card); border:1px solid var(--border);
    border-radius:14px; padding:32px;
  }}
  .top-row {{
    display:flex; justify-content:space-between; align-items:flex-start;
    margin-bottom:12px; gap:12px;
  }}
  .brand {{ display:flex; align-items:center; gap:10px; }}
  h1 {{ font-size:28px; font-weight:700; letter-spacing:-0.7px; }}
  p {{ margin:12px 0; font-size:14px; }}
  a {{ color:var(--fg); text-decoration:underline; }}
  hr {{ border:none; border-top:1px solid var(--border); margin:22px 0; }}
  .badge {{
    display:inline-block; background:var(--bg); border:1px solid var(--border);
    color:var(--muted); padding:3px 10px; border-radius:8px; font-size:12px; font-weight:500;
  }}
  .nav-links {{ display:flex; gap:8px; margin-top:20px; }}
  .btn-link {{
    text-decoration:none; font-size:13px; color:var(--fg);
    background:var(--bg); border:1px solid var(--border);
    padding:7px 14px; border-radius:8px; font-weight:500;
  }}
  .btn-link:hover {{ border-color:var(--border-hover); }}
</style></head>
<body>
  <div class="card">
    <div class="top-row">
      <div class="brand">
        <svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="12 2 19 21 12 17 5 21 12 2"/>
        </svg>
        <h1>Kestrel Browser</h1>
      </div>
      <span class="badge">v1.0.0-alpha · Servo 0.6</span>
    </div>
    <p>A lightweight, pure-Rust, zero-telemetry web browser for Linux,
       built on the <a href="https://servo.org">Servo engine</a> with native GTK4 UI and network-level ad blocking via <code>adblock-rust</code>.</p>
    <p><strong>License:</strong> GNU General Public License v3.0</p>
    <p><strong>Repository:</strong>
       <a href="https://github.com/LocShadowVN/kestrel-browser">github.com/LocShadowVN/kestrel-browser</a></p>
    <hr>
    <h2 style="font-size:17px;margin-bottom:8px;">Tiếng Việt</h2>
    <p>Trình duyệt web nhẹ, thuần Rust, không thu thập dữ liệu (zero-telemetry) dành cho Linux,
       xây dựng trên nhân <a href="https://servo.org">Servo 0.6</a> cùng bộ chặn quảng cáo Kestrel Shields.</p>
    <p><strong>Giấy phép:</strong> GNU General Public License v3.0</p>
    <div class="nav-links">
      <a class="btn-link" href="kestrel://home">← {home_lbl}</a>
      <a class="btn-link" href="kestrel://settings">{settings_lbl}</a>
    </div>
  </div>
  <script>{shim}</script>
</body></html>"#,
        lang = lang,
        theme_class = theme_class,
        home_lbl = home_lbl,
        settings_lbl = settings_lbl,
        shim = NAV_SHIM,
    )
}

/// Giải mã JSON an toàn kể cả khi chuỗi bị bọc 2 lớp (`"\"{\\\"action\\\":...}\""`).
fn parse_ipc_json(body: &str) -> Option<serde_json::Value> {
    let first = serde_json::from_str::<serde_json::Value>(body).ok()?;
    if let serde_json::Value::String(inner) = first {
        serde_json::from_str::<serde_json::Value>(&inner).ok()
    } else {
        Some(first)
    }
}

struct TabEntry {
    tab_id: u64,
    webview: WebView,
    logical_url: Rc<RefCell<String>>,
    title: Rc<RefCell<String>>,
    blocked_count: Rc<Cell<u32>>,
    loading: Rc<Cell<bool>>,
    blob_synced: Rc<Cell<bool>>,
    #[allow(dead_code)]
    ucm: UserContentManager,
}

pub struct MainWindow {
    window: ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &Application, db: Rc<Database>, config: Rc<Config>) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Kestrel Browser")
            .default_width(1400)
            .default_height(900)
            .build();
        window.add_css_class("kestrel-window");
        apply_window_theme(&window, config.dark_theme());

        let tabs: Rc<RefCell<Vec<TabEntry>>> = Rc::new(RefCell::new(Vec::new()));
        let active_tab_id: Rc<Cell<u64>> = Rc::new(Cell::new(0));
        let total_blocked: Rc<Cell<u64>> = Rc::new(Cell::new(0));

        let tabbar = Rc::new(TabBar::new());
        let toolbar = Rc::new(Toolbar::new({
            let tabs_c = tabs.clone();
            let config_c = config.clone();
            move |level: u8| {
                log::info!("Adblock level changed to {level}");
                if let Err(e) = config_c.set_adblock_level(level) {
                    log::warn!("Failed to save adblock level: {e}");
                }
                let maybe_blob = crate::adblock_rules::try_get_blob();
                for tab in tabs_c.borrow().iter() {
                    let cur = tab.logical_url.borrow().clone();
                    if !is_internal_or_about(&cur) {
                        tab.webview.set_adblock_level(level as i32);
                        if level > 0 && !tab.blob_synced.get() {
                            if let Some(blob) = maybe_blob {
                                if !blob.is_empty() {
                                    tab.webview.set_adblock_blob(blob.to_vec());
                                    tab.blob_synced.set(true);
                                }
                            }
                        }
                    }
                }
            }
        }));
        toolbar.set_adblock_level(config.adblock_level());

        // ---- Toolbar: Back ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            toolbar.on_back(move || {
                let id = active_c.get();
                if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                    tab.webview.go_back();
                }
            });
        }

        // ---- Toolbar: Forward ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            toolbar.on_forward(move || {
                let id = active_c.get();
                if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                    tab.webview.go_forward();
                }
            });
        }

        // ---- Toolbar: Reload ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let toolbar_c = toolbar.clone();
            let tabbar_c = tabbar.clone();
            let window_c = window.clone();
            let total_c = total_blocked.clone();
            toolbar.on_reload(move || {
                let id = active_c.get();
                if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                    reload_tab(
                        tab,
                        &db_c,
                        &config_c,
                        &toolbar_c,
                        &tabbar_c,
                        &window_c,
                        &total_c,
                    );
                }
            });
        }

        // ---- Omnibox Enter → Navigate ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let toolbar_c = toolbar.clone();
            let tabbar_c = tabbar.clone();
            let window_c = window.clone();
            let total_c = total_blocked.clone();
            toolbar.omnibox().on_activate(move |text| {
                let search_engine = config_c.search_engine();
                let url = navigation::normalize_url_with_search(text, &search_engine);
                let id = active_c.get();
                if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                    navigate_tab(
                        tab,
                        &url,
                        &db_c,
                        &config_c,
                        &toolbar_c,
                        &tabbar_c,
                        &window_c,
                        &total_c,
                        true,
                    );
                }
            });
        }

        // ---- Home button ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let toolbar_c = toolbar.clone();
            let tabbar_c = tabbar.clone();
            let window_c = window.clone();
            let total_c = total_blocked.clone();
            toolbar.on_home(move || {
                let home = config_c.homepage();
                let id = active_c.get();
                if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                    navigate_tab(
                        tab,
                        &home,
                        &db_c,
                        &config_c,
                        &toolbar_c,
                        &tabbar_c,
                        &window_c,
                        &total_c,
                        true,
                    );
                }
            });
        }

        // ---- Omnibox Star Button: Toggle Bookmark ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            let db_c = db.clone();
            let toolbar_c = toolbar.clone();
            toolbar.on_bookmark(move || {
                toggle_active_bookmark(&tabs_c, active_c.get(), &db_c, &toolbar_c);
            });
        }

        // ---- Toolbar Bookmarks Quick Button ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let toolbar_c = toolbar.clone();
            let tabbar_c = tabbar.clone();
            let window_c = window.clone();
            let total_c = total_blocked.clone();
            toolbar.on_bookmarks_button(move || {
                let id = active_c.get();
                if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                    navigate_tab(
                        tab,
                        "kestrel://bookmarks",
                        &db_c,
                        &config_c,
                        &toolbar_c,
                        &tabbar_c,
                        &window_c,
                        &total_c,
                        true,
                    );
                }
            });
        }

        // ---- Tabbar: nút "+" ----
        {
            let tabs_c = tabs.clone();
            let tabbar_c = tabbar.clone();
            let toolbar_c = toolbar.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let active_c = active_tab_id.clone();
            let total_c = total_blocked.clone();
            let window_c = window.clone();
            tabbar.on_new_tab(move || {
                let home = config_c.homepage();
                Self::open_tab(
                    &window_c,
                    &tabs_c,
                    &tabbar_c,
                    &toolbar_c,
                    &db_c,
                    &active_c,
                    &total_c,
                    &config_c,
                    &home,
                );
            });
        }

        // ---- Menu: New Tab ----
        {
            let tabs_c = tabs.clone();
            let tabbar_c = tabbar.clone();
            let toolbar_c = toolbar.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let active_c = active_tab_id.clone();
            let total_c = total_blocked.clone();
            let window_c = window.clone();
            toolbar.on_new_tab(move || {
                let home = config_c.homepage();
                Self::open_tab(
                    &window_c,
                    &tabs_c,
                    &tabbar_c,
                    &toolbar_c,
                    &db_c,
                    &active_c,
                    &total_c,
                    &config_c,
                    &home,
                );
            });
        }

        // ---- Menu: Bookmarks ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let toolbar_c = toolbar.clone();
            let tabbar_c = tabbar.clone();
            let window_c = window.clone();
            let total_c = total_blocked.clone();
            toolbar.on_bookmarks(move || {
                let id = active_c.get();
                if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                    navigate_tab(
                        tab,
                        "kestrel://bookmarks",
                        &db_c,
                        &config_c,
                        &toolbar_c,
                        &tabbar_c,
                        &window_c,
                        &total_c,
                        true,
                    );
                }
            });
        }

        // ---- Menu: History ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let toolbar_c = toolbar.clone();
            let tabbar_c = tabbar.clone();
            let window_c = window.clone();
            let total_c = total_blocked.clone();
            toolbar.on_history(move || {
                let id = active_c.get();
                if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                    navigate_tab(
                        tab,
                        "kestrel://history",
                        &db_c,
                        &config_c,
                        &toolbar_c,
                        &tabbar_c,
                        &window_c,
                        &total_c,
                        true,
                    );
                }
            });
        }

        // ---- Menu: Settings ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let toolbar_c = toolbar.clone();
            let tabbar_c = tabbar.clone();
            let window_c = window.clone();
            let total_c = total_blocked.clone();
            toolbar.on_settings(move || {
                let id = active_c.get();
                if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                    navigate_tab(
                        tab,
                        "kestrel://settings",
                        &db_c,
                        &config_c,
                        &toolbar_c,
                        &tabbar_c,
                        &window_c,
                        &total_c,
                        true,
                    );
                }
            });
        }

        // ---- Menu: About ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let toolbar_c = toolbar.clone();
            let tabbar_c = tabbar.clone();
            let window_c = window.clone();
            let total_c = total_blocked.clone();
            toolbar.on_about(move || {
                let id = active_c.get();
                if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                    navigate_tab(
                        tab,
                        "kestrel://about",
                        &db_c,
                        &config_c,
                        &toolbar_c,
                        &tabbar_c,
                        &window_c,
                        &total_c,
                        true,
                    );
                }
            });
        }

        // ---- Menu: Quit ----
        {
            let window_c = window.clone();
            toolbar.on_quit(move || {
                window_c.close();
            });
        }

        // ---- Tab switch ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab_id.clone();
            let toolbar_c = toolbar.clone();
            let db_c = db.clone();
            let total_c = total_blocked.clone();
            let window_c = window.clone();
            tabbar.on_switch(move |tab_id| {
                active_c.set(tab_id);
                if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == tab_id) {
                    sync_chrome_for_tab(tab, &toolbar_c, &db_c, &window_c, total_c.get());
                }
            });
        }

        // ---- Keyboard Shortcuts ----
        {
            let key_ctrl = EventControllerKey::new();
            key_ctrl.set_propagation_phase(PropagationPhase::Capture);

            let window_c = window.clone();
            let tabs_c = tabs.clone();
            let tabbar_c = tabbar.clone();
            let toolbar_c = toolbar.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let active_c = active_tab_id.clone();
            let total_c = total_blocked.clone();

            key_ctrl.connect_key_pressed(move |_, keyval, _keycode, state| {
                let ctrl = state.contains(gdk::ModifierType::CONTROL_MASK);
                let shift = state.contains(gdk::ModifierType::SHIFT_MASK);
                let alt = state.contains(gdk::ModifierType::ALT_MASK);

                if keyval == gdk::Key::F11 {
                    if window_c.is_fullscreen() {
                        window_c.unfullscreen();
                    } else {
                        window_c.fullscreen();
                    }
                    return glib::Propagation::Stop;
                }

                if keyval == gdk::Key::F5 || (ctrl && matches!(keyval, gdk::Key::r | gdk::Key::R)) {
                    let id = active_c.get();
                    if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                        reload_tab(
                            tab,
                            &db_c,
                            &config_c,
                            &toolbar_c,
                            &tabbar_c,
                            &window_c,
                            &total_c,
                        );
                    }
                    return glib::Propagation::Stop;
                }

                if alt && keyval == gdk::Key::Left {
                    let id = active_c.get();
                    if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                        tab.webview.go_back();
                    }
                    return glib::Propagation::Stop;
                }

                if alt && keyval == gdk::Key::Right {
                    let id = active_c.get();
                    if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                        tab.webview.go_forward();
                    }
                    return glib::Propagation::Stop;
                }

                if alt && keyval == gdk::Key::Home {
                    let home = config_c.homepage();
                    let id = active_c.get();
                    if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                        navigate_tab(
                            tab,
                            &home,
                            &db_c,
                            &config_c,
                            &toolbar_c,
                            &tabbar_c,
                            &window_c,
                            &total_c,
                            true,
                        );
                    }
                    return glib::Propagation::Stop;
                }

                if (alt && matches!(keyval, gdk::Key::d | gdk::Key::D))
                    || (ctrl && matches!(keyval, gdk::Key::l | gdk::Key::L))
                {
                    toolbar_c.omnibox().focus_and_select();
                    return glib::Propagation::Stop;
                }

                if ctrl {
                    match keyval {
                        gdk::Key::t | gdk::Key::T => {
                            let home = config_c.homepage();
                            Self::open_tab(
                                &window_c,
                                &tabs_c,
                                &tabbar_c,
                                &toolbar_c,
                                &db_c,
                                &active_c,
                                &total_c,
                                &config_c,
                                &home,
                            );
                            toolbar_c.omnibox().focus_and_select();
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::w | gdk::Key::W => {
                            let id = active_c.get();
                            Self::close_tab_by_id(
                                id,
                                &window_c,
                                &tabs_c,
                                &tabbar_c,
                                &toolbar_c,
                                &db_c,
                                &active_c,
                                &total_c,
                                &config_c,
                            );
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::d | gdk::Key::D => {
                            toggle_active_bookmark(&tabs_c, active_c.get(), &db_c, &toolbar_c);
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::h | gdk::Key::H => {
                            let id = active_c.get();
                            if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                                navigate_tab(
                                    tab,
                                    "kestrel://history",
                                    &db_c,
                                    &config_c,
                                    &toolbar_c,
                                    &tabbar_c,
                                    &window_c,
                                    &total_c,
                                    true,
                                );
                            }
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::b | gdk::Key::B => {
                            let id = active_c.get();
                            if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                                navigate_tab(
                                    tab,
                                    "kestrel://bookmarks",
                                    &db_c,
                                    &config_c,
                                    &toolbar_c,
                                    &tabbar_c,
                                    &window_c,
                                    &total_c,
                                    true,
                                );
                            }
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::comma => {
                            let id = active_c.get();
                            if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == id) {
                                navigate_tab(
                                    tab,
                                    "kestrel://settings",
                                    &db_c,
                                    &config_c,
                                    &toolbar_c,
                                    &tabbar_c,
                                    &window_c,
                                    &total_c,
                                    true,
                                );
                            }
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::q | gdk::Key::Q => {
                            window_c.close();
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::Tab
                        | gdk::Key::ISO_Left_Tab
                        | gdk::Key::Page_Down
                        | gdk::Key::Page_Up => {
                            let count = tabbar_c.count();
                            if count > 1 {
                                let cur = tabbar_c.active_index();
                                let prev = shift
                                    || keyval == gdk::Key::ISO_Left_Tab
                                    || keyval == gdk::Key::Page_Up;
                                let next = if prev {
                                    (cur + count - 1) % count
                                } else {
                                    (cur + 1) % count
                                };
                                tabbar_c.set_active(next);
                            }
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::_1 => {
                            tabbar_c.set_active(0);
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::_2 => {
                            tabbar_c.set_active(1);
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::_3 => {
                            tabbar_c.set_active(2);
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::_4 => {
                            tabbar_c.set_active(3);
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::_5 => {
                            tabbar_c.set_active(4);
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::_6 => {
                            tabbar_c.set_active(5);
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::_7 => {
                            tabbar_c.set_active(6);
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::_8 => {
                            tabbar_c.set_active(7);
                            return glib::Propagation::Stop;
                        }
                        gdk::Key::_9 => {
                            let n = tabbar_c.count();
                            if n > 0 {
                                tabbar_c.set_active(n - 1);
                            }
                            return glib::Propagation::Stop;
                        }
                        _ => {}
                    }
                }

                glib::Propagation::Proceed
            });

            window.add_controller(key_ctrl);
        }

        // ---- Layout: tab strip → toolbar → content ----
        let vbox = GtkBox::new(Orientation::Vertical, 0);
        vbox.append(tabbar.strip_widget());
        vbox.append(toolbar.widget());
        vbox.append(tabbar.content_widget());
        window.set_child(Some(&vbox));

        window.present();

        // ---- Mở tab đầu tiên ----
        {
            let homepage = config.homepage();
            Self::open_tab(
                &window,
                &tabs,
                &tabbar,
                &toolbar,
                &db,
                &active_tab_id,
                &total_blocked,
                &config,
                &homepage,
            );
        }

        Self { window }
    }

    #[allow(clippy::too_many_arguments)]
    fn close_tab_by_id(
        tab_id: u64,
        window: &ApplicationWindow,
        tabs: &Rc<RefCell<Vec<TabEntry>>>,
        tabbar: &Rc<TabBar>,
        toolbar: &Rc<Toolbar>,
        db: &Rc<Database>,
        active_tab_id: &Rc<Cell<u64>>,
        total_blocked: &Rc<Cell<u64>>,
        config: &Rc<Config>,
    ) {
        tabbar.close_tab(tab_id);
        tabs.borrow_mut().retain(|t| t.tab_id != tab_id);

        if tabs.borrow().is_empty() {
            let home = config.homepage();
            Self::open_tab(
                window,
                tabs,
                tabbar,
                toolbar,
                db,
                active_tab_id,
                total_blocked,
                config,
                &home,
            );
        } else if let Some(new_active) = tabbar.active_tab_id() {
            active_tab_id.set(new_active);
            if let Some(tab) = tabs.borrow().iter().find(|t| t.tab_id == new_active) {
                sync_chrome_for_tab(tab, toolbar, db, window, total_blocked.get());
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn open_tab(
        window: &ApplicationWindow,
        tabs: &Rc<RefCell<Vec<TabEntry>>>,
        tabbar: &Rc<TabBar>,
        toolbar: &Rc<Toolbar>,
        db: &Rc<Database>,
        active_tab_id: &Rc<Cell<u64>>,
        total_blocked: &Rc<Cell<u64>>,
        config: &Rc<Config>,
        url: &str,
    ) {
        let ucm = UserContentManager::new();
        // QUAN TRỌNG: Đăng ký handler "kestrel" TRƯỚC KHI nạp script để `window.servoGtk.messageHandlers.kestrel` luôn sẵn sàng
        ucm.register_script_message_handler("kestrel");
        ucm.add_script(&UserScript::new(crate::compat::POLYFILL_SCRIPT));

        let webview = WebView::with_user_content_manager(&ucm);
        webview.set_hexpand(true);
        webview.set_vexpand(true);

        let logical_url = Rc::new(RefCell::new(url.to_string()));
        let tab_title = Rc::new(RefCell::new("New Tab".to_string()));
        let blocked_count = Rc::new(Cell::new(0u32));
        let loading = Rc::new(Cell::new(false));
        let blob_synced = Rc::new(Cell::new(false));

        let tab_id = {
            let window_c = window.clone();
            let tabs_c = tabs.clone();
            let tabbar_c = tabbar.clone();
            let toolbar_c = toolbar.clone();
            let db_c = db.clone();
            let active_c = active_tab_id.clone();
            let total_c = total_blocked.clone();
            let config_c = config.clone();
            tabbar.add_tab(&webview, "New Tab", move |closed_id| {
                Self::close_tab_by_id(
                    closed_id,
                    &window_c,
                    &tabs_c,
                    &tabbar_c,
                    &toolbar_c,
                    &db_c,
                    &active_c,
                    &total_c,
                    &config_c,
                );
            })
        };

        tabs.borrow_mut().push(TabEntry {
            tab_id,
            webview: webview.clone(),
            logical_url: logical_url.clone(),
            title: tab_title.clone(),
            blocked_count: blocked_count.clone(),
            loading: loading.clone(),
            blob_synced: blob_synced.clone(),
            ucm: ucm.clone(),
        });
        active_tab_id.set(tab_id);

        // ---- Message handler từ các trang nội bộ & Cosmetic Adblock ----
        {
            let window_c = window.clone();
            let tabs_c = tabs.clone();
            let tabbar_c = tabbar.clone();
            let toolbar_c = toolbar.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let active_c = active_tab_id.clone();
            let total_c = total_blocked.clone();
            let blocked_c = blocked_count.clone();
            ucm.connect_script_message_received(move |_ucm, _name, body| {
                let Some(v) = parse_ipc_json(body) else {
                    log::warn!("Invalid IPC JSON: {}", body);
                    return;
                };
                let action = v.get("action").and_then(|a| a.as_str()).unwrap_or("");
                let target_id = tab_id;
                let is_active = active_c.get() == target_id;

                match action {
                    "adblock-cosmetic-hit" => {
                        if config_c.adblock_level() == 0 {
                            return;
                        }
                        let hit = v
                            .get("count")
                            .and_then(|c| c.as_u64())
                            .unwrap_or(1)
                            .min(500) as u32;
                        let new_tab_count = blocked_c.get().saturating_add(hit);
                        let new_total = total_c.get().saturating_add(hit as u64);
                        blocked_c.set(new_tab_count);
                        total_c.set(new_total);
                        if is_active {
                            toolbar_c
                                .adblock()
                                .set_blocked_count(new_tab_count, new_total);
                        }
                    }
                    "navigate" => {
                        if let Some(u) = v.get("url").and_then(|u| u.as_str()) {
                            let search_engine = config_c.search_engine();
                            let normalized =
                                navigation::normalize_url_with_search(u, &search_engine);
                            if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == target_id)
                            {
                                navigate_tab(
                                    tab,
                                    &normalized,
                                    &db_c,
                                    &config_c,
                                    &toolbar_c,
                                    &tabbar_c,
                                    &window_c,
                                    &total_c,
                                    is_active,
                                );
                            }
                        }
                    }
                    "save-settings" => {
                        let old_lang = config_c.language();
                        if let Some(h) = v.get("homepage").and_then(|x| x.as_str()) {
                            let _ = config_c.set_homepage(h);
                        }
                        if let Some(s) = v
                            .get("search_engine")
                            .or_else(|| v.get("search"))
                            .and_then(|x| x.as_str())
                        {
                            let _ = config_c.set_search_engine(s);
                        }
                        if let Some(ua) = v.get("user_agent").and_then(|x| x.as_str()) {
                            let _ = config_c.set_user_agent(ua);
                        }
                        if let Some(l) = v.get("language").and_then(|x| x.as_str()) {
                            let _ = config_c.set_language(l);
                        }
                        if let Some(d) = v
                            .get("dark_theme")
                            .or_else(|| v.get("dark"))
                            .and_then(|x| x.as_bool())
                        {
                            let _ = config_c.set_dark_theme(d);
                            apply_window_theme(&window_c, d);
                        }
                        if let Some(ab) = v.get("adblock_level").and_then(|x| {
                            x.as_u64()
                                .or_else(|| x.as_str().and_then(|s| s.parse::<u64>().ok()))
                        }) {
                            let level = (ab as u8).min(2);
                            let _ = config_c.set_adblock_level(level);
                            toolbar_c.set_adblock_level(level);
                            let maybe_blob = crate::adblock_rules::try_get_blob();
                            for t in tabs_c.borrow().iter() {
                                let cur = t.logical_url.borrow().clone();
                                if !is_internal_or_about(&cur) {
                                    t.webview.set_adblock_level(level as i32);
                                    if level > 0 && !t.blob_synced.get() {
                                        if let Some(blob) = maybe_blob {
                                            if !blob.is_empty() {
                                                t.webview.set_adblock_blob(blob.to_vec());
                                                t.blob_synced.set(true);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if config_c.language() != old_lang {
                            if let Some(tab) =
                                tabs_c.borrow().iter().find(|t| t.tab_id == target_id)
                            {
                                let cur = tab.logical_url.borrow().clone();
                                if internal_pages::classify(&cur) == Some(Page::Settings) {
                                    navigate_tab(
                                        tab,
                                        "kestrel://settings",
                                        &db_c,
                                        &config_c,
                                        &toolbar_c,
                                        &tabbar_c,
                                        &window_c,
                                        &total_c,
                                        is_active,
                                    );
                                }
                            }
                        }
                    }
                    "reset-settings" => {
                        let _ = config_c.set_homepage("kestrel://home");
                        let _ = config_c.set_search_engine("https://search.brave.com/search?q=");
                        let _ = config_c.set_user_agent(shared::DEFAULT_USER_AGENT);
                        let _ = config_c.set_language("en");
                        let _ = config_c.set_dark_theme(true);
                        let _ = config_c.set_adblock_level(1);
                        apply_window_theme(&window_c, true);
                        toolbar_c.set_adblock_level(1);
                        for t in tabs_c.borrow().iter() {
                            let cur = t.logical_url.borrow().clone();
                            if !is_internal_or_about(&cur) {
                                t.webview.set_adblock_level(1);
                            }
                        }
                        if let Some(tab) = tabs_c.borrow().iter().find(|t| t.tab_id == target_id) {
                            navigate_tab(
                                tab,
                                "kestrel://settings",
                                &db_c,
                                &config_c,
                                &toolbar_c,
                                &tabbar_c,
                                &window_c,
                                &total_c,
                                is_active,
                            );
                        }
                    }
                    "add-bookmark" => {
                        if let (Some(u), Some(t)) = (
                            v.get("url").and_then(|x| x.as_str()),
                            v.get("title").and_then(|x| x.as_str()),
                        ) {
                            let norm = navigation::normalize_url(u);
                            let title = if t.trim().is_empty() { &norm } else { t.trim() };
                            if let Err(e) = db_c.insert_bookmark(&norm, title) {
                                log::warn!("Add bookmark failed: {e}");
                            } else if let Some(tab) =
                                tabs_c.borrow().iter().find(|x| x.tab_id == target_id)
                            {
                                navigate_tab(
                                    tab,
                                    "kestrel://bookmarks",
                                    &db_c,
                                    &config_c,
                                    &toolbar_c,
                                    &tabbar_c,
                                    &window_c,
                                    &total_c,
                                    is_active,
                                );
                            }
                        }
                    }
                    "remove-bookmark" => {
                        if let Some(id) = v.get("id").and_then(|x| x.as_i64()) {
                            let _ = db_c.remove_bookmark(id);
                        }
                    }
                    "clear-bookmarks" => {
                        let _ = db_c.clear_bookmarks();
                        if let Some(tab) = tabs_c.borrow().iter().find(|x| x.tab_id == target_id) {
                            let cur = tab.logical_url.borrow().clone();
                            navigate_tab(
                                tab,
                                &cur,
                                &db_c,
                                &config_c,
                                &toolbar_c,
                                &tabbar_c,
                                &window_c,
                                &total_c,
                                is_active,
                            );
                        }
                    }
                    "clear-history" => {
                        if let Err(e) = db_c.clear_history() {
                            log::warn!("Clear history failed: {e}");
                        } else if let Some(tab) =
                            tabs_c.borrow().iter().find(|x| x.tab_id == target_id)
                        {
                            let cur = tab.logical_url.borrow().clone();
                            if internal_pages::classify(&cur) == Some(Page::History) {
                                navigate_tab(
                                    tab,
                                    "kestrel://history",
                                    &db_c,
                                    &config_c,
                                    &toolbar_c,
                                    &tabbar_c,
                                    &window_c,
                                    &total_c,
                                    is_active,
                                );
                            }
                        }
                    }
                    "delete-history" => {
                        if let Some(id) = v.get("id").and_then(|x| x.as_i64()) {
                            let _ = db_c.delete_history_item(id);
                        }
                    }
                    _ => log::warn!("Unknown action: {}", action),
                }
            });
        }

        // ---- Title changed ----
        {
            let tabbar_c = tabbar.clone();
            let window_c = window.clone();
            let active_c = active_tab_id.clone();
            let title_c = tab_title.clone();
            webview.connect_title_notify(move |wv| {
                let title = wv
                    .title()
                    .filter(|t| !t.trim().is_empty())
                    .unwrap_or_else(|| "Untitled".into());
                *title_c.borrow_mut() = title.clone();
                tabbar_c.set_tab_title(tab_id, &title);
                if active_c.get() == tab_id {
                    window_c.set_title(Some(&format!("{} — Kestrel Browser", title)));
                }
            });
        }

        // ---- URI changed ----
        {
            let toolbar_c = toolbar.clone();
            let active_c = active_tab_id.clone();
            let logical_c = logical_url.clone();
            let blocked_c = blocked_count.clone();
            let total_c = total_blocked.clone();
            let db_c = db.clone();
            webview.connect_uri_notify(move |wv| {
                if let Some(uri) = wv.uri() {
                    if uri.starts_with("data:") {
                        return;
                    }
                    let current_logical = logical_c.borrow().clone();
                    if uri == "about:blank" && is_internal_or_about(&current_logical) {
                        return;
                    }
                    *logical_c.borrow_mut() = uri.clone();
                    if active_c.get() == tab_id {
                        toolbar_c.omnibox().set_text(&uri);
                        toolbar_c.omnibox().set_bookmarked(db_c.is_bookmarked(&uri));
                        toolbar_c
                            .adblock()
                            .set_blocked_count(blocked_c.get(), total_c.get());
                    }
                }
            });
        }

        // ---- Load state changed (CHỈ đồng bộ adblock_blob khi tải trang mạng http/https) ----
        {
            let db_c = db.clone();
            let tabbar_c = tabbar.clone();
            let toolbar_c = toolbar.clone();
            let active_c = active_tab_id.clone();
            let logical_c = logical_url.clone();
            let loading_c = loading.clone();
            let config_c = config.clone();
            let synced_c = blob_synced.clone();
            webview.connect_load_changed(move |wv, event| {
                let current_logical = logical_c.borrow().clone();
                let is_internal = is_internal_or_about(&current_logical);

                if !is_internal && !synced_c.get() {
                    if let Some(blob) = crate::adblock_rules::try_get_blob() {
                        let lv = config_c.adblock_level();
                        wv.set_adblock_level(lv as i32);
                        if !blob.is_empty() {
                            wv.set_adblock_blob(blob.to_vec());
                            synced_c.set(true);
                        }
                    }
                }

                match event {
                    LoadEvent::Finished => {
                        if loading_c.get() {
                            loading_c.set(false);
                            tabbar_c.set_tab_loading(tab_id, false, is_internal);
                        }
                        if active_c.get() == tab_id {
                            toolbar_c.set_loading(false);
                        }
                        if let Some(uri) = wv.uri() {
                            if uri.starts_with("http://") || uri.starts_with("https://") {
                                let title = wv
                                    .title()
                                    .filter(|t| !t.trim().is_empty())
                                    .unwrap_or_else(|| uri.clone());
                                let _ = db_c.insert_history(&uri, &title);
                            }
                        }
                    }
                    _ => {
                        if !loading_c.get() {
                            loading_c.set(true);
                            tabbar_c.set_tab_loading(tab_id, true, is_internal);
                        }
                        if active_c.get() == tab_id {
                            toolbar_c.set_loading(true);
                        }
                    }
                }
            });
        }

        // ---- Adblock blocked (Network level) ----
        {
            let blocked_c = blocked_count.clone();
            let total_c = total_blocked.clone();
            let active_c = active_tab_id.clone();
            let toolbar_c = toolbar.clone();
            webview.connect_adblock_blocked(move |_wv, url, resource_type| {
                let new_tab_count = blocked_c.get().saturating_add(1);
                let new_total = total_c.get().saturating_add(1);
                blocked_c.set(new_tab_count);
                total_c.set(new_total);
                log::debug!(
                    "[Shields] Blocked #{new_tab_count} (total {new_total}): {url} ({resource_type})"
                );
                if active_c.get() == tab_id {
                    toolbar_c
                        .adblock()
                        .set_blocked_count(new_tab_count, new_total);
                }
            });
        }

        // ---- Popups (`target="_blank"` / `window.open`) ----
        {
            let window_c = window.clone();
            let tabs_c = tabs.clone();
            let tabbar_c = tabbar.clone();
            let toolbar_c = toolbar.clone();
            let db_c = db.clone();
            let active_c = active_tab_id.clone();
            let total_c = total_blocked.clone();
            let config_c = config.clone();
            webview.connect_create_web_view(move |_wv, popup_url| {
                if !popup_url.trim().is_empty() {
                    Self::open_tab(
                        &window_c,
                        &tabs_c,
                        &tabbar_c,
                        &toolbar_c,
                        &db_c,
                        &active_c,
                        &total_c,
                        &config_c,
                        &popup_url,
                    );
                }
            });
        }

        if let Some(entry) = tabs.borrow().iter().find(|t| t.tab_id == tab_id) {
            navigate_tab(
                entry,
                url,
                db,
                config,
                toolbar,
                tabbar,
                window,
                total_blocked,
                true,
            );
        }

        tabbar.set_active_by_id(tab_id);
    }

    pub fn present(&self) {
        self.window.present();
    }
}

fn is_about_url(url: &str) -> bool {
    let clean = url
        .trim()
        .split(&['?', '#'][..])
        .next()
        .unwrap_or("")
        .trim_end_matches('/');
    clean.eq_ignore_ascii_case("kestrel://about")
}

fn is_internal_or_about(url: &str) -> bool {
    internal_pages::is_internal(url) || is_about_url(url)
}

fn apply_window_theme(window: &ApplicationWindow, dark: bool) {
    if dark {
        window.remove_css_class("theme-light");
    } else {
        window.add_css_class("theme-light");
    }
}

fn sync_chrome_for_tab(
    tab: &TabEntry,
    toolbar: &Toolbar,
    db: &Database,
    window: &ApplicationWindow,
    session_blocked: u64,
) {
    let logical = tab.logical_url.borrow().clone();
    let display_url = if !logical.is_empty() && is_internal_or_about(&logical) {
        logical
    } else if let Some(uri) = tab.webview.uri() {
        if uri.starts_with("data:") || (uri == "about:blank" && !logical.is_empty()) {
            logical
        } else {
            uri
        }
    } else {
        logical
    };

    toolbar.omnibox().set_text(&display_url);
    toolbar
        .omnibox()
        .set_bookmarked(db.is_bookmarked(&display_url));
    toolbar
        .adblock()
        .set_blocked_count(tab.blocked_count.get(), session_blocked);
    toolbar.set_loading(tab.loading.get());

    let title = tab.title.borrow().clone();
    if title.trim().is_empty() {
        window.set_title(Some("Kestrel Browser"));
    } else {
        window.set_title(Some(&format!("{} — Kestrel Browser", title)));
    }
}

#[allow(clippy::too_many_arguments)]
fn reload_tab(
    tab: &TabEntry,
    db: &Database,
    config: &Config,
    toolbar: &Toolbar,
    tabbar: &TabBar,
    window: &ApplicationWindow,
    total_blocked: &Rc<Cell<u64>>,
) {
    let current_url = tab.logical_url.borrow().clone();
    if is_internal_or_about(&current_url) {
        navigate_tab(
            tab,
            &current_url,
            db,
            config,
            toolbar,
            tabbar,
            window,
            total_blocked,
            true,
        );
    } else {
        tab.blocked_count.set(0);
        toolbar.adblock().set_blocked_count(0, total_blocked.get());
        tab.webview.reload();
    }
}

fn toggle_active_bookmark(
    tabs: &Rc<RefCell<Vec<TabEntry>>>,
    active_id: u64,
    db: &Database,
    toolbar: &Toolbar,
) {
    if let Some(tab) = tabs.borrow().iter().find(|t| t.tab_id == active_id) {
        let logical = tab.logical_url.borrow().clone();
        let url = if !logical.is_empty() && !logical.starts_with("data:") {
            logical
        } else if let Some(u) = tab.webview.uri() {
            u
        } else {
            return;
        };

        if url.is_empty()
            || url.starts_with("data:")
            || url == "about:blank"
            || is_internal_or_about(&url)
        {
            return;
        }

        if db.is_bookmarked(&url) {
            if let Err(e) = db.remove_bookmark_by_url(&url) {
                log::warn!("Unbookmark failed: {e}");
            } else {
                toolbar.omnibox().set_bookmarked(false);
            }
        } else {
            let title = tab
                .webview
                .title()
                .filter(|t| !t.trim().is_empty())
                .unwrap_or_else(|| url.clone());
            if let Err(e) = db.insert_bookmark(&url, &title) {
                log::warn!("Bookmark failed: {e}");
            } else {
                toolbar.omnibox().set_bookmarked(true);
            }
        }
    }
}

fn set_tab_and_window_title(
    tab: &TabEntry,
    tabbar: &TabBar,
    window: &ApplicationWindow,
    title: &str,
    is_active: bool,
) {
    *tab.title.borrow_mut() = title.to_string();
    tabbar.set_tab_title(tab.tab_id, title);
    if is_active {
        window.set_title(Some(&format!("{} — Kestrel Browser", title)));
    }
}

#[allow(clippy::too_many_arguments)]
fn navigate_tab(
    tab: &TabEntry,
    url: &str,
    db: &Database,
    config: &Config,
    toolbar: &Toolbar,
    tabbar: &TabBar,
    window: &ApplicationWindow,
    total_blocked: &Rc<Cell<u64>>,
    is_active: bool,
) {
    let clean_url = if let Some(q) = internal_pages::query_string(url) {
        if internal_pages::classify(url) == Some(Page::Settings) {
            apply_settings_query(q, config, window, toolbar);
            "kestrel://settings"
        } else {
            url
        }
    } else if url.trim().is_empty() || url.trim() == "about:blank" {
        "kestrel://home"
    } else {
        url
    };

    *tab.logical_url.borrow_mut() = clean_url.to_string();
    tab.blocked_count.set(0);

    if is_active {
        toolbar.omnibox().set_text(clean_url);
        toolbar
            .omnibox()
            .set_bookmarked(db.is_bookmarked(clean_url));
        toolbar.adblock().set_blocked_count(0, total_blocked.get());
    }

    let is_internal = is_internal_or_about(clean_url);
    if is_internal {
        tabbar.set_tab_favicon(tab.tab_id, "web-browser-symbolic");
    } else {
        tabbar.set_tab_favicon(tab.tab_id, "text-x-generic-symbolic");
    }

    let lang = config.language();
    let dark = config.dark_theme();

    if is_about_url(clean_url) {
        let html = render_about_html(&lang, dark);
        tab.webview.load_html(&html, None);
        set_tab_and_window_title(tab, tabbar, window, "About Kestrel", is_active);
        return;
    }

    match internal_pages::classify(clean_url) {
        Some(Page::Home) => {
            let html = internal_pages::home::render_with_stats(
                &lang,
                dark,
                db.bookmark_count(),
                db.history_count(),
                config.adblock_level(),
                total_blocked.get(),
            );
            tab.webview.load_html(&html, None);
            set_tab_and_window_title(tab, tabbar, window, "New Tab", is_active);
        }
        Some(Page::Settings) => {
            let html = internal_pages::settings::render_full(
                &lang,
                &config.homepage(),
                &config.search_engine(),
                dark,
                config.adblock_level(),
                &config.user_agent(),
            );
            tab.webview.load_html(&html, None);
            set_tab_and_window_title(tab, tabbar, window, "Settings", is_active);
        }
        Some(Page::History) => {
            let records = db
                .fetch_history(250)
                .unwrap_or_default()
                .into_iter()
                .map(|r| (r.id.unwrap_or(0), r.url, r.title, r.timestamp))
                .collect::<Vec<_>>();
            let html = internal_pages::history::render(&records, &lang, dark);
            tab.webview.load_html(&html, None);
            set_tab_and_window_title(tab, tabbar, window, "History", is_active);
        }
        Some(Page::Bookmarks) => {
            let records = db
                .fetch_bookmarks()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|r| r.id.map(|id| (id, r.url, r.title)))
                .collect::<Vec<_>>();
            let html = internal_pages::bookmarks::render(&records, &lang, dark);
            tab.webview.load_html(&html, None);
            set_tab_and_window_title(tab, tabbar, window, "Bookmarks", is_active);
        }
        None => {
            let lv = config.adblock_level();
            tab.webview.set_adblock_level(lv as i32);
            if lv > 0 && !tab.blob_synced.get() {
                if let Some(blob) = crate::adblock_rules::try_get_blob() {
                    if !blob.is_empty() {
                        tab.webview.set_adblock_blob(blob.to_vec());
                        tab.blob_synced.set(true);
                    }
                }
            }
            tab.webview.load_url(clean_url);
        }
    }
}

fn apply_settings_query(
    query: &str,
    config: &Config,
    window: &ApplicationWindow,
    toolbar: &Toolbar,
) {
    use std::collections::HashMap;
    let params: HashMap<String, String> = url::form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect();
    if params.contains_key("reset") {
        let _ = config.set_homepage("kestrel://home");
        let _ = config.set_search_engine("https://search.brave.com/search?q=");
        let _ = config.set_user_agent(shared::DEFAULT_USER_AGENT);
        let _ = config.set_language("en");
        let _ = config.set_dark_theme(true);
        let _ = config.set_adblock_level(1);
        apply_window_theme(window, true);
        toolbar.set_adblock_level(1);
        return;
    }
    if let Some(h) = params.get("homepage") {
        let _ = config.set_homepage(h);
    }
    if let Some(s) = params.get("search_engine").or_else(|| params.get("search")) {
        let _ = config.set_search_engine(s);
    }
    if let Some(ua) = params.get("user_agent") {
        let _ = config.set_user_agent(ua);
    }
    if let Some(l) = params.get("language") {
        let _ = config.set_language(l);
    }
    if let Some(d) = params.get("dark_theme").or_else(|| params.get("dark")) {
        let is_dark = d == "true" || d == "1" || d == "on";
        let _ = config.set_dark_theme(is_dark);
        apply_window_theme(window, is_dark);
    }
    if let Some(ab) = params
        .get("adblock_level")
        .and_then(|v| v.parse::<u8>().ok())
    {
        let _ = config.set_adblock_level(ab);
        toolbar.set_adblock_level(ab);
    }
}
