//! MainWindow — layout: tabbar trên, toolbar dưới, web content dưới cùng.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation};

use servo_gtk::user_content::{UserContentManager, UserScript};
use servo_gtk::{LoadEvent, WebView};

use crate::chrome::tabbar::TabBar;
use crate::chrome::toolbar::Toolbar;
use crate::engine::navigation;
use crate::internal_pages::{self, Page};
use crate::storage::database::Database;
use crate::util::config::Config;

const ABOUT_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>About Kestrel</title>
<style>
  :root { --bg:#fafafa; --fg:#1a1a1a; --muted:#6b6b6b; --accent:#0a84ff; --code:#eee; }
  @media (prefers-color-scheme: dark) {
    :root { --bg:#1c1c1e; --fg:#f5f5f7; --muted:#a0a0a8; --code:#2c2c2e; }
  }
  * { box-sizing:border-box; margin:0; padding:0; }
  body { font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;
         background:var(--bg); color:var(--fg); padding:48px 24px;
         max-width:640px; margin:0 auto; line-height:1.6; }
  h1 { font-size:36px; font-weight:700; letter-spacing:-1px; margin-bottom:4px; }
  h1 span { color:var(--accent); }
  p { margin:10px 0; }
  .muted { color:var(--muted); font-size:14px; }
  a { color:var(--accent); text-decoration:none; }
  a:hover { text-decoration:underline; }
  hr { border:none; border-top:1px solid var(--code); margin:32px 0; }
</style></head>
<body>
  <h1>Kestrel<span>.</span></h1>
  <p class="muted">Version 0.1.0-alpha</p>
  <p>A lightweight, pure-Rust, zero-telemetry web browser for Linux,
     built on the <a href="https://servo.org">Servo engine</a>.</p>
  <p><strong>License:</strong> GNU General Public License v3.0</p>
  <p><strong>Source:</strong>
     <a href="https://github.com/LocShadowVN/kestrel-browser">github.com/LocShadowVN/kestrel-browser</a></p>
  <hr>
  <h1 style="font-size:24px;">Tiếng Việt</h1>
  <p>Trình duyệt nhẹ, thuần Rust, không telemetry cho Linux,
     xây trên <a href="https://servo.org">Servo engine</a>.</p>
  <p><strong>Giấy phép:</strong> GNU General Public License v3.0</p>
</body></html>"#;

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

        // ---- Toolbar callbacks ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            toolbar.on_back(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    tab.webview.go_back();
                }
            });
        }
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            toolbar.on_forward(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    tab.webview.go_forward();
                }
            });
        }
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            toolbar.on_reload(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    tab.webview.reload();
                }
            });
        }
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            toolbar.omnibox().on_activate(move |text| {
                let url = navigation::normalize_url(text);
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    load_any(&tab.webview, &url, &db_c, &config_c);
                }
            });
        }
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            toolbar.on_home(move || {
                let home = config_c.homepage();
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    load_any(&tab.webview, &home, &db_c, &config_c);
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
            let active_c = active_tab.clone();
            tabbar.on_new_tab(move || {
                let home = config_c.homepage();
                Self::open_tab(
                    &tabs_c,
                    &tabbar_c,
                    &toolbar_c,
                    &db_c,
                    &active_c,
                    &config_c,
                    &home,
                );
            });
        }

        // ---- Menu callbacks ----
        {
            let tabs_c = tabs.clone();
            let tabbar_c = tabbar.clone();
            let toolbar_c = toolbar.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let active_c = active_tab.clone();
            toolbar.on_new_tab(move || {
                let home = config_c.homepage();
                Self::open_tab(
                    &tabs_c,
                    &tabbar_c,
                    &toolbar_c,
                    &db_c,
                    &active_c,
                    &config_c,
                    &home,
                );
            });
        }
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            toolbar.on_history(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    load_any(&tab.webview, "kestrel://history", &db_c, &config_c);
                }
            });
        }
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            toolbar.on_settings(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    load_any(&tab.webview, "kestrel://settings", &db_c, &config_c);
                }
            });
        }
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            toolbar.on_about(move || {
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    tab.webview.load_html(ABOUT_HTML, None);
                }
            });
        }
        {
            toolbar.on_quit(|| {
                std::process::exit(0);
            });
        }

        // ---- Tab switch ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let omnibox_c = toolbar.omnibox().clone();
            tabbar.on_switch(move |page_num| {
                active_c.set(page_num as usize);
                if let Some(tab) = tabs_c.borrow().get(page_num as usize) {
                    if let Some(uri) = tab.webview.uri() {
                        if uri.starts_with("data:") {
                            omnibox_c.set_text("kestrel://home");
                        } else {
                            omnibox_c.set_text(&uri);
                        }
                    }
                }
            });
        }

        // ---- Layout: tabbar trên, toolbar dưới, content cuối ----
        let vbox = GtkBox::new(Orientation::Vertical, 0);
        vbox.append(tabbar.widget());
        vbox.append(toolbar.widget());
        window.set_child(Some(&vbox));

                // ---- Mở tab đầu tiên NGAY, không đợi window map ----
        //
        // Trước đây dùng connect_map — nhưng connect_map chạy sau khi window
        // được present, nên user thấy cửa sổ trắng 1-2 giây trước khi Servo
        // runner spawn và render home. Mở tab ngay trong constructor thì
        // WebView bắt đầu load song song với quá trình GTK show window,
        // giảm thời gian thấy trắng.
        {
            let homepage = config.homepage();
            Self::open_tab(
                &tabs,
                &tabbar,
                &toolbar,
                &db,
                &active_tab,
                &config,
                &homepage,
            );
        }

        Self { window }
    }

    #[allow(clippy::too_many_arguments)]
    fn open_tab(
        tabs: &Rc<RefCell<Vec<TabEntry>>>,
        tabbar: &Rc<TabBar>,
        toolbar: &Rc<Toolbar>,
        db: &Rc<Database>,
        active_tab: &Rc<Cell<usize>>,
        config: &Rc<Config>,
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

        // Message handler
        {
            let tabs_c = tabs.clone();
            let db_c = db.clone();
            let config_c = config.clone();
            let active_c = active_tab.clone();
            ucm.connect_script_message_received(move |_ucm, _name, body| {
                log::info!("Message from page: {}", body);
                let Ok(v) = serde_json::from_str::<serde_json::Value>(body) else {
                    log::warn!("Invalid JSON: {}", body);
                    return;
                };
                let action = v.get("action").and_then(|a| a.as_str()).unwrap_or("");
                match action {
                    "navigate" => {
                        if let Some(u) = v.get("url").and_then(|u| u.as_str()) {
                            let normalized = navigation::normalize_url(u);
                            let idx = active_c.get();
                            if let Some(tab) = tabs_c.borrow().get(idx) {
                                load_any(&tab.webview, &normalized, &db_c, &config_c);
                            }
                        }
                    }
                    "save-settings" => {
                        if let Some(h) = v.get("homepage").and_then(|x| x.as_str()) {
                            let _ = config_c.set_homepage(h);
                        }
                        if let Some(s) = v.get("search_engine").and_then(|x| x.as_str()) {
                            let _ = config_c.set_search_engine(s);
                        }
                        if let Some(l) = v.get("language").and_then(|x| x.as_str()) {
                            let _ = config_c.set_language(l);
                        }
                        if let Some(d) = v.get("dark_theme").and_then(|x| x.as_bool()) {
                            let _ = config_c.set_dark_theme(d);
                        }
                        log::info!("Settings saved.");
                    }
                    _ => log::warn!("Unknown action: {}", action),
                }
            });
        }

        // Title change
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

        // URI change
        {
            let omnibox_c = toolbar.omnibox().clone();
            let active_c = active_tab.clone();
            webview.connect_uri_notify(move |wv| {
                if active_c.get() != tab_index {
                    return;
                }
                if let Some(uri) = wv.uri() {
                    if uri.starts_with("data:") {
                        return;
                    }
                    omnibox_c.set_text(&uri);
                }
            });
        }

        // History record
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

        // Popups
        {
            webview.connect_create_web_view(move |_wv, url| {
                log::info!("Popup requested: {}", url);
            });
        }

        load_any(&webview, url, db, config);
        toolbar.omnibox().set_text(url);

        tabs.borrow_mut().push(TabEntry {
            webview,
            page_num,
            ucm,
        });
        active_tab.set(tab_index);
    }

    pub fn present(&self) {
        self.window.present();
    }
}

/// Load URL, tự xử lý trang nội bộ.
fn load_any(webview: &WebView, url: &str, db: &Database, config: &Config) {
    let lang = config.language();

    match internal_pages::classify(url) {
        Some(Page::Home) => {
            log::debug!("Loading internal home ({})", lang);
            let html = internal_pages::home::render(&lang);
            webview.load_html(&html, None);
        }
        Some(Page::Settings) => {
            log::debug!("Loading internal settings");
            let html = internal_pages::settings::render(
                &lang,
                &config.homepage(),
                &config.search_engine(),
                config.dark_theme(),
            );
            webview.load_html(&html, None);
        }
        Some(Page::History) => {
            log::debug!("Loading internal history");
            let records = db
                .fetch_history(200)
                .unwrap_or_default()
                .into_iter()
                .map(|r| (r.url, r.title, r.timestamp))
                .collect::<Vec<_>>();
            let html = internal_pages::history::render(&records);
            webview.load_html(&html, None);
        }
        None => {
            log::debug!("Loading URL: {}", url);
            webview.load_url(url);
        }
    }
}
