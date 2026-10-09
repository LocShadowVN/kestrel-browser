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

/// State của một tab.
struct TabEntry {
    webview: WebView,
    page_num: u32,
    /// UserContentManager của tab này. Giữ để không bị drop.
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
                    tab.webview.go_back();
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
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    load_any(&tab.webview, &home, &db_c);
                }
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
                        omnibox_c.set_text(&uri);
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
            ucm.connect_script_message_received(move |_ucm, _name, body| {
                log::debug!("Message from page: {}", body);
                match serde_json::from_str::<serde_json::Value>(body) {
                    Ok(v) => {
                        let action = v.get("action").and_then(|a| a.as_str()).unwrap_or("");
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
        toolbar.omnibox().set_text(url);

        tabs.borrow_mut().push(TabEntry { webview, page_num, ucm });
        active_tab.set(tab_index);
    }

    pub fn present(&self) {
        self.window.present();
    }
}

/// Load một URL vào WebView, tự động xử lý trang nội bộ.
///
/// - `kestrel://home`, `kestrel://settings` → HTML tĩnh
/// - `kestrel://history` → HTML động từ database
/// - URL khác → `load_url()` bình thường
fn load_any(webview: &WebView, url: &str, db: &Database) {
    // Trang tĩnh
    if let Some(html) = internal_pages::resolve(url) {
        webview.load_html(html, Some("kestrel://"));
        return;
    }

    // Trang động
    if internal_pages::is_dynamic(url) {
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
    webview.load_url(url);
}
