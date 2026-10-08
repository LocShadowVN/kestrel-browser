//! Toolbar — navigation buttons + omnibox + menu.

use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, MenuButton, Orientation};

use crate::chrome::omnibox::Omnibox;
use crate::engine::browser::BrowserState;
use crate::engine::navigation;

pub struct Toolbar {
    container: GtkBox,
    omnibox: Rc<Omnibox>,
}

impl Toolbar {
    pub fn new(browser: Rc<BrowserState>) -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 4);
        container.set_margin_top(4);
        container.set_margin_bottom(4);
        container.set_margin_start(4);
        container.set_margin_end(4);

        let back_btn = Button::from_icon_name("go-previous-symbolic");
        let forward_btn = Button::from_icon_name("go-next-symbolic");
        let reload_btn = Button::from_icon_name("view-refresh-symbolic");
        let home_btn = Button::from_icon_name("go-home-symbolic");
        let menu_btn = MenuButton::new();
        menu_btn.set_icon_name("open-menu-symbolic");

        container.append(&back_btn);
        container.append(&forward_btn);
        container.append(&reload_btn);

        // Omnibox
        let omnibox = Rc::new(Omnibox::new());
        container.append(omnibox.widget());

        container.append(&home_btn);
        container.append(&menu_btn);

        // Callback: Enter trong omnibox → navigate
        let browser_nav = browser.clone();
        omnibox.set_on_submit(move |text| {
            let url = navigation::normalize_url(text);
            browser_nav.navigate(&url);
        });

        // Callback: Back
        let browser_back = browser.clone();
        back_btn.connect_clicked(move |_| {
            browser_back.go_back();
        });

        // Callback: Forward
        let browser_fwd = browser.clone();
        forward_btn.connect_clicked(move |_| {
            browser_fwd.go_forward();
        });

        // Callback: Reload
        let browser_reload = browser.clone();
        reload_btn.connect_clicked(move |_| {
            browser_reload.reload();
        });

        // Callback: Home
        let browser_home = browser.clone();
        home_btn.connect_clicked(move |_| {
            let home = browser_home.homepage();
            browser_home.navigate(&home);
        });

        Self { container, omnibox }
    }

    pub fn widget(&self) -> &GtkBox {
        &self.container
    }

    /// Update omnibox khi URL thay đổi (gọi từ delegate).
    pub fn set_url(&self, url: &str) {
        self.omnibox.set_text(url);
    }

    /// Focus omnibox (Ctrl+L).
    pub fn focus_omnibox(&self) {
        self.omnibox.focus();
    }
}
