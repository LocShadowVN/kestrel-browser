//! Browser state — quản lý tabs, navigation, history.

use std::cell::RefCell;
use std::rc::Rc;

use servo_gtk::WebView;

use crate::chrome::tabbar::TabBar;
use crate::storage::database::Database;
use crate::util::config::Config;

pub struct Tab {
    pub webview: WebView,
    pub title: String,
    pub url: String,
    pub page_num: u32,
}

pub struct BrowserState {
    db: Rc<Database>,
    config: Rc<Config>,
    tabs: RefCell<Vec<Tab>>,
    active_tab: RefCell<usize>,
}

impl BrowserState {
    pub fn new(db: Rc<Database>, config: Rc<Config>) -> Self {
        Self {
            db,
            config,
            tabs: RefCell::new(Vec::new()),
            active_tab: RefCell::new(0),
        }
    }

    pub fn homepage(&self) -> String {
        self.config.homepage()
    }

    /// Mở tab mới trong TabBar.
    pub fn open_tab(&self, tabbar: &TabBar, url: &str) {
        let webview = WebView::new();
        webview.set_vexpand(true);
        webview.set_hexpand(true);
        webview.load_url(url);

        let page_num = tabbar.add_tab(&webview, "Loading...");

        let tab = Tab {
            webview,
            title: "New Tab".into(),
            url: url.into(),
            page_num,
        };

        {
            let mut tabs = self.tabs.borrow_mut();
            tabs.push(tab);
            *self.active_tab.borrow_mut() = tabs.len() - 1;
        }

        let _ = self.db.insert_history(url, url);
    }

    pub fn set_active_tab(&self, index: usize) {
        *self.active_tab.borrow_mut() = index;
    }

    pub fn navigate(&self, url: &str) {
        let active = *self.active_tab.borrow();
        let tabs = self.tabs.borrow();
        if let Some(tab) = tabs.get(active) {
            tab.webview.load_url(url);
            let _ = self.db.insert_history(url, url);
        }
    }

    pub fn go_back(&self) {
        // TODO: Servo API for history navigation chưa verify.
        // Cần đọc source servo-gtk để biết method chính xác.
        log::info!("go_back: chưa implement");
    }

    pub fn go_forward(&self) {
        // TODO: Servo API for history navigation chưa verify.
        log::info!("go_forward: chưa implement");
    }

    pub fn reload(&self) {
        let active = *self.active_tab.borrow();
        let tabs = self.tabs.borrow();
        if let Some(tab) = tabs.get(active) {
            // Servo-gtk chưa có reload() rõ ràng.
            // Workaround: load lại URL hiện tại.
            tab.webview.load_url(&tab.url);
            log::info!("reload: {}", tab.url);
        }
    }
}
