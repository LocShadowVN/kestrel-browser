//! MainWindow — layout: toolbar trên, tabbar dưới.
//!
//! Quản lý tabs, kết nối signals từ Servo WebView → UI, xử lý điều hướng
//! đến các trang nội bộ (kestrel://home, kestrel://settings, kestrel://history).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation};

use servo_gtk::user_content::{UserContentManager, UserScript};
use servo_gtk::{LoadEvent, WebView};

use crate::chrome::tabbar::TabBar;
use crate::chrome::toolbar::Toolbar;
use crate::engine::navigation;
use crate::internal_pages;
use crate::storage::database::Database;
use crate::util::config::Config;

/// HTML cho trang About — song ngữ, English first.
const ABOUT_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>About Kestrel</title>
<style>
  :root { --bg: #fafafa; --fg: #1a1a1a; --muted: #6b6b6b; --accent: #0a84ff; --code-bg: #eee; }
  @media (prefers-color-scheme: dark) {
    :root { --bg: #1c1c1e; --fg: #f5f5f7; --muted: #a0a0a8; --code-bg: #2c2c2e; }
  }
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    background: var(--bg); color: var(--fg);
    padding: 48px 24px; max-width: 640px; margin: 0 auto; line-height: 1.6;
  }
  h1 { font-size: 36px; font-weight: 700; letter-spacing: -1px; margin-bottom: 4px; }
  h1 span { color: var(--accent); }
  h2 { font-size: 18px; font-weight: 600; margin: 32px 0 8px; color: var(--muted); }
  p { margin: 8px 0; color: var(--fg); }
  .muted { color: var(--muted); font-size: 14px; }
  code { background: var(--code-bg); padding: 2px 6px; border-radius: 4px;
         font-family: "SF Mono", Consolas, monospace; font-size: 13px; }
  a { color: var(--accent); text-decoration: none; }
  a:hover { text-decoration: underline; }
  hr { border: none; border-top: 1px solid var(--code-bg); margin: 32px 0; }
</style>
</head>
<body>
  <h1>Kestrel<span>.</span></h1>
  <p class="muted">Version 0.1.0-alpha</p>

  <p>A lightweight, pure-Rust, zero-telemetry web browser for Linux,
     built on the <a href="https://servo.org">Servo engine</a>.</p>

  <p><strong>License:</strong> GNU General Public License v3.0</p>
  <p><strong>Source:</strong>
     <a href="https://github.com/LocShadowVN/kestrel-browser">github.com/LocShadowVN/kestrel-browser</a></p>

  <hr>

  <h2>Tiếng Việt</h2>
  <p>Trình duyệt nhẹ, thuần Rust, không telemetry cho Linux,
     xây trên <a href="https://servo.org">Servo engine</a>.</p>
  <p><strong>Giấy phép:</strong> GNU General Public License v3.0</p>
  <p><strong>Mã nguồn:</strong>
     <a href="https://github.com/LocShadowVN/kestrel-browser">github.com/LocShadowVN/kestrel-browser</a></p>
</body>
</html>"#;

/// State của một tab.
struct TabEntry {
    webview: WebView,
    page_num: u32,
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

        let toolbar = Rc::new(Toolbar::new());
        let tabbar = Rc::new(TabBar::new());

        let tabs: Rc<RefCell<Vec<TabEntry>>> = Rc::new(RefCell::new(Vec::new()));
        let active_tab: Rc<Cell<usize>> = Rc::new(Cell::new(0));

        // ---- Toolbar: back ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            toolbar.on_back(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    log::debug!("Back button pressed for tab {}", idx);
                    tab.webview.go_back();
                } else {
                    log::debug!("Back button pressed but no tab at index {}", idx);
                }
            });
        }

        // ---- Toolbar: forward ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            toolbar.on_forward(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    log::debug!("Forward button pressed for tab {}", idx);
                    tab.webview.go_forward();
                }
            });
        }

        // ---- Toolbar: reload ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            toolbar.on_reload(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    log::debug!("Reload button pressed for tab {}", idx);
                    tab.webview.reload();
                }
            });
        }

        // ---- Omnibox Enter → navigate ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let db_c = db.clone();
            toolbar.omnibox().on_activate(move |text| {
                log::debug!("Omnibox activated: {}", text);
                let url = navigation::normalize_url(text);
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    load_any(&tab.webview, &url, &db_c);
                }
            });
        }

        // ---- Home button ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let config_c = config.clone();
            let db_c = db.clone();
            toolbar.on_home(move || {
                let home = config_c.homepage();
                log::debug!("Home button pressed, loading {}", home);
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    load_any(&tab.webview, &home, &db_c);
                }
            });
        }

        // ---- Menu: New Tab (loads home in active tab for now) ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            toolbar.on_new_tab(move || {
                let home = config_c.homepage();
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    load_any(&tab.webview, &home, &db_c);
                }
            });
        }

        // ---- Menu: History ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let db_c = db.clone();
            toolbar.on_history(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    load_any(&tab.webview, "kestrel://history", &db_c);
                }
            });
        }

        // ---- Menu: Settings ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let db_c = db.clone();
            toolbar.on_settings(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    load_any(&tab.webview, "kestrel://settings", &db_c);
                }
            });
        }

        // ---- Menu: About ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            toolbar.on_about(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    tab.webview.load_html(ABOUT_HTML, Some("kestrel://about"));
                }
            });
        }

        // ---- Menu: Quit ----
        {
            toolbar.on_quit(|| {
                log::info!("Quit requested from menu");
                std::process::exit(0);
            });
        }

        // ---- Tab switch → update omnibox ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let omnibox_c = toolbar.omnibox().clone();
            tabbar.on_switch(move |page_num| {
                active_c.set(page_num as usize);
                if let Some(tab) = tabs_c.borrow().get(page_num as usize) {
                    if let Some(uri) = tab.webview.uri() {
                        // Bỏ qua data: URL — implementation detail của load_html().
                        if uri.starts_with("data:") {
                            omnibox_c.set_text("kestrel://home");
                        } else {
                            omnibox_c.set_text(&uri);
                        }
                    }
                }
            });
        }

        // ---- Layout ----
        let vbox = GtkBox::new(Orientation::Vertical, 0);
        vbox.append(toolbar.widget());
        vbox.append(tabbar.widget());
        window.set_child(Some(&vbox));

        // ---- Open first tab after window is mapped ----
        {
            let tabs_c = tabs.clone();
            let tabbar_c = tabbar.clone();
            let toolbar_c = toolbar.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let active_c = active_tab.clone();

            window.connect_map(move |_| {
                if !tabs_c.borrow().is_empty() {
                    return;
                }
                let homepage = config_c.homepage();
                Self::open_tab(
                    &tabs_c,
                    &tabbar_c,
                    &toolbar_c,
                    &db_c,
                    &active_c,
                    &homepage,
                );
            });
        }

        Self { window }
    }

    fn open_tab(
        tabs: &Rc<RefCell<Vec<TabEntry>>>,
        tabbar: &Rc<TabBar>,
        toolbar: &Rc<Toolbar>,
        db: &Rc<Database>,
        active_tab: &Rc<Cell<usize>>,
        url: &str,
    ) {
        let ucm = UserContentManager::new();
        ucm.add_script(&UserScript::new(crate::compat::POLYFILL_SCRIPT));
        ucm.register_script_message_handler("kestrel");

        let webview = WebView::with_user_content_manager(&ucm);
        webview.set_hexpand(true);
        webview.set_vexpand(true);

        let page_num = tabbar.add_tab(&webview, "Loading...");
        let tab_index = tabs.borrow().len();

        // ---- Message handler ----
        {
            let tabs_c = tabs.clone();
            let db_c = db.clone();
            let active_c = active_tab.clone();
            ucm.connect_script_message_received(move |_ucm, name, body| {
                log::info!("Message from page [{}]: {}", name, body);
                match serde_json::from_str::<serde_json::Value>(body) {
                    Ok(v) => {
                        let action = v.get("action").and_then(|a| a.as_str()).unwrap_or("");
                        log::info!("Action: {}", action);
                        match action {
                            "navigate" => {
                                if let Some(u) = v.get("url").and_then(|u| u.as_str()) {
                                    let normalized = navigation::normalize_url(u);
                                    let idx = active_c.get();
                                    if let Some(tab) = tabs_c.borrow().get(idx) {
                                        load_any(&tab.webview, &normalized, &db_c);
                                    }
                                }
                            }
                            "save-settings" => {
                                if let Some(h) = v.get("homepage").and_then(|x| x.as_str()) {
                                    let _ = db_c.save_config_item("homepage", h);
                                }
                                if let Some(s) = v.get("search_engine").and_then(|x| x.as_str()) {
                                    let _ = db_c.save_config_item("search_engine", s);
                                }
                                if let Some(d) = v.get("dark_theme").and_then(|x| x.as_bool()) {
                                    let _ = db_c.save_config_item(
                                        "dark_theme",
                                        if d { "true" } else { "false" },
                                    );
                                }
                                log::info!("Settings saved from page.");
                            }
                            _ => {
                                log::warn!("Unknown action from page: {}", action);
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Invalid JSON from page: {} ({})", body, e);
                    }
                }
            });
        }

        // ---- Signal: title changed → update tab label ----
        {
            let tabbar_c = tabbar.clone();
            webview.connect_title_notify(move |wv| {
                let title = wv
                    .title()
                    .filter(|t| !t.is_empty())
                    .unwrap_or_else(|| "Untitled".into());
                tabbar_c.set_tab_title(page_num, &title);
            });
        }

        // ---- Signal: uri changed → update omnibox ----
        {
            let omnibox_c = toolbar.omnibox().clone();
            let active_c = active_tab.clone();
            webview.connect_uri_notify(move |wv| {
                if active_c.get() != tab_index {
                    return;
                }
                if let Some(uri) = wv.uri() {
                    // Bỏ qua data: URL — implementation detail của load_html().
                    if uri.starts_with("data:") {
                        return;
                    }
                    omnibox_c.set_text(&uri);
                }
            });
        }

        // ---- Signal: load finished → record history ----
        {
            let db_c = db.clone();
            webview.connect_load_changed(move |wv, event| {
                if let LoadEvent::Finished = event {
                    if let (Some(uri), Some(title)) = (wv.uri(), wv.title()) {
                        if uri.starts_with("http://")
                            || uri.starts_with("https://")
                            || uri.starts_with("kestrel://")
                        {
                            let _ = db_c.insert_history(&uri, &title);
                        }
                    }
                }
            });
        }

        // ---- Popups ----
        {
            webview.connect_create_web_view(move |_wv, url| {
                log::info!("Popup requested: {}", url);
            });
        }

        // ---- Load nội dung ban đầu ----
        load_any(&webview, url, db);

        // Set omnibox text — nếu là trang nội bộ, hiển thị kestrel://
        toolbar.omnibox().set_text(url);

        tabs.borrow_mut().push(TabEntry { webview, page_num, ucm });
        active_tab.set(tab_index);
    }

    pub fn present(&self) {
        self.window.present();
    }
}

/// Load một URL vào WebView, tự động xử lý trang nội bộ.
fn load_any(webview: &WebView, url: &str, db: &Database) {
    // Trang tĩnh
    if let Some(html) = internal_pages::resolve(url) {
        log::debug!("Loading internal static page: {}", url);
        webview.load_html(html, Some("kestrel://"));
        return;
    }

    // Trang động
    if internal_pages::is_dynamic(url) {
        log::debug!("Loading internal dynamic page: {}", url);
        let records = match db.fetch_history(200) {
            Ok(rows) => rows
                .into_iter()
                .map(|r| (r.url, r.title, r.timestamp))
                .collect::<Vec<_>>(),
            Err(e) => {
                log::warn!("Failed to fetch history: {}", e);
                Vec::new()
            }
        };
        let html = internal_pages::history::render(&records);
        webview.load_html(&html, Some("kestrel://"));
        return;
    }

    // URL bình thường
    log::debug!("Loading URL: {}", url);
    webview.load_url(url);
}
