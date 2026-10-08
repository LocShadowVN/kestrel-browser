//! MainWindow — layout: toolbar trên, tabbar dưới.
//!
//! Quản lý tabs, kết nối signals từ Servo WebView → UI.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation};

use servo_gtk::{LoadEvent, WebView};

use crate::chrome::tabbar::TabBar;
use crate::chrome::toolbar::Toolbar;
use crate::engine::navigation;
use crate::storage::database::Database;
use crate::util::config::Config;

/// State của một tab.
struct TabEntry {
    webview: WebView,
    page_num: u32,
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

        // ---- Wire toolbar actions ----
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

        // ---- Omnibox Enter → navigate ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            toolbar.omnibox().on_activate(move |text| {
                let url = navigation::normalize_url(text);
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    tab.webview.load_url(&url);
                }
            });
        }

        // ---- Home button ----
        {
            let tabs_c = tabs.clone();
            let active_c = active_tab.clone();
            let config_c = config.clone();
            toolbar.on_home(move || {
                let home = config_c.homepage();
                let idx = active_c.get();
                if let Some(tab) = tabs_c.borrow().get(idx) {
                    tab.webview.load_url(&home);
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

    /// Tạo tab mới, kết nối signals, thêm vào Notebook.
    fn open_tab(
        tabs: &Rc<RefCell<Vec<TabEntry>>>,
        tabbar: &Rc<TabBar>,
        toolbar: &Rc<Toolbar>,
        db: &Rc<Database>,
        active_tab: &Rc<Cell<usize>>,
        url: &str,
    ) {
        let webview = WebView::new();
        webview.set_hexpand(true);
        webview.set_vexpand(true);

        let page_num = tabbar.add_tab(&webview, "Loading...");
        let tab_index = tabs.borrow().len();

        // Signal: title changed → update tab label
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

        // Signal: uri changed → update omnibox if this is active tab
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

        // Signal: load finished → record history
        {
            let db_c = db.clone();
            webview.connect_load_changed(move |wv, event| {
                if let LoadEvent::Finished = event {
                    if let (Some(uri), Some(title)) = (wv.uri(), wv.title()) {
                        if uri.starts_with("http://") || uri.starts_with("https://") {
                            let _ = db_c.insert_history(&uri, &title);
                        }
                    }
                }
            });
        }

        // Popups: window.open / target=_blank → log only (Phase 2: mở tab mới)
        {
            webview.connect_create_web_view(move |_wv, url| {
                log::info!("Popup requested: {}", url);
            });
        }

        webview.load_url(url);

        tabs.borrow_mut().push(TabEntry { webview, page_num });
        active_tab.set(tab_index);

        // Sync omnibox with initial URL
        toolbar.omnibox().set_text(url);
    }

    pub fn present(&self) {
        self.window.present();
    }
}
