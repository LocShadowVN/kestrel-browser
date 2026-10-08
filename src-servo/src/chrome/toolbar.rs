use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Entry, Orientation};
use std::rc::Rc;

use crate::engine::browser::BrowserState;
use crate::engine::navigation;

pub struct Toolbar {
    container: GtkBox,
}

impl Toolbar {
    pub fn new(browser: Rc<BrowserState>) -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 4);
        container.set_margin_top(4);
        container.set_margin_bottom(4);
        container.set_margin_start(4);
        container.set_margin_end(4);

        // Navigation buttons
        let back_btn = Button::from_icon_name("go-previous-symbolic");
        let forward_btn = Button::from_icon_name("go-next-symbolic");
        let reload_btn = Button::from_icon_name("view-refresh-symbolic");
        let home_btn = Button::from_icon_name("go-home-symbolic");

        container.append(&back_btn);
        container.append(&forward_btn);
        container.append(&reload_btn);

        // Omnibox
        let omnibox = Entry::new();
        omnibox.set_hexpand(true);
        omnibox.set_placeholder_text(Some("Search or enter URL"));
        container.append(&omnibox);

        container.append(&home_btn);

        // Connect: Back
        let browser_back = browser.clone();
        back_btn.connect_clicked(move |_| {
            browser_back.go_back();
        });

        // Connect: Forward
        let browser_fwd = browser.clone();
        forward_btn.connect_clicked(move |_| {
            browser_fwd.go_forward();
        });

        // Connect: Reload
        let browser_reload = browser.clone();
        reload_btn.connect_clicked(move |_| {
            browser_reload.reload();
        });

        // Connect: Home
        let browser_home = browser.clone();
        home_btn.connect_clicked(move |_| {
            browser_home.navigate("about:blank");
        });

        // Connect: Omnibox Enter
        let browser_nav = browser.clone();
        let omnibox_for_back = omnibox.clone();
        omnibox.connect_activate(move |entry| {
            let text = entry.text().to_string();
            let url = navigation::normalize_url(&text);
            browser_nav.navigate(&url);
        });

        // Update omnibox when URL changes
        let omnibox_update = omnibox.clone();
        // TODO: kết nối với delegate để cập nhật URL

        Self { container }
    }

    pub fn widget(&self) -> &GtkBox {
        &self.container
    }
}
