use gtk4::prelude::*;
use gtk4::Notebook;
use std::rc::Rc;
use std::cell::RefCell;

use servo_gtk::WebView;

use crate::storage::database::Database;

pub struct Tab {
    pub webview: WebView,
    pub title: String,
    pub url: String,
}

pub struct BrowserState {
    db: Rc<Database>,
    tabs: RefCell<Vec<Tab>>,
    active_tab: RefCell<usize>,
}

impl BrowserState {
    pub fn new(db: Rc<Database>) -> Self {
        Self {
            db,
            tabs: RefCell::new(Vec::new()),
            active_tab: RefCell::new(0),
        }
    }

    pub fn open_tab_in_notebook(&self, notebook: &Notebook, url: &str) {
        let webview = WebView::new();
        webview.set_vexpand(true);
        webview.set_hexpand(true);
        webview.load_url(url);

        let tab = Tab {
            webview: webview.clone(),
            title: "New Tab".into(),
            url: url.into(),
        };

        let tabs_len = {
            let mut tabs = self.tabs.borrow_mut();
            tabs.push(tab);
            tabs.len()
        };

        let page_num = notebook.append_page(&webview, Some(&gtk4::Label::new(Some("New Tab"))));
        notebook.set_current_page(Some(page_num));
        *self.active_tab.borrow_mut() = tabs_len - 1;

        // Lưu history
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
        // TODO: Servo API cho back/forward
    }

    pub fn go_forward(&self) {
        // TODO: Servo API cho back/forward
    }

    pub fn reload(&self) {
        let active = *self.active_tab.borrow();
        let tabs = self.tabs.borrow();
        if let Some(tab) = tabs.get(active) {
            tab.webview.load_url(&tab.url);
        }
    }
}
