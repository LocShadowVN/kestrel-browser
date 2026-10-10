//! Toolbar — navigation buttons + omnibox + hamburger menu.

use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, MenuButton, Orientation, Popover, Separator,
};

use crate::chrome::omnibox::Omnibox;

pub struct Toolbar {
    container: GtkBox,
    omnibox: Rc<Omnibox>,
    back: Button,
    forward: Button,
    reload: Button,
    home: Button,
    menu: MenuButton,
    menu_new_tab: Button,
    menu_history: Button,
    menu_settings: Button,
    menu_about: Button,
    menu_quit: Button,
}

impl Toolbar {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 0);
        container.add_css_class("kestrel-toolbar");
        container.set_spacing(4);

        // ---- Navigation buttons ----
        let back = make_icon_button("go-previous-symbolic", "Quay lại");
        let forward = make_icon_button("go-next-symbolic", "Tiến");
        let reload = make_icon_button("view-refresh-symbolic", "Tải lại");

        // ---- Omnibox ----
        let omnibox = Rc::new(Omnibox::new());

        // ---- Right side ----
        let home = make_icon_button("go-home-symbolic", "Trang chủ");

        // ---- Hamburger menu ----
        let menu_button_inner = Button::with_label("New Tab");
        menu_button_inner.add_css_class("kestrel-menu-item");
        let menu_history = Button::with_label("Lịch sử");
        menu_history.add_css_class("kestrel-menu-item");
        let menu_settings = Button::with_label("Cài đặt");
        menu_settings.add_css_class("kestrel-menu-item");
        let menu_about = Button::with_label("Giới thiệu");
        menu_about.add_css_class("kestrel-menu-item");
        let menu_quit = Button::with_label("Thoát");
        menu_quit.add_css_class("kestrel-menu-item");

        let menu_box = GtkBox::new(Orientation::Vertical, 0);
        menu_box.add_css_class("kestrel-menu");
        menu_box.append(&menu_button_inner);
        menu_box.append(&menu_history);
        menu_box.append(&menu_settings);
        let sep = Separator::new(Orientation::Horizontal);
        sep.add_css_class("kestrel-menu-separator");
        menu_box.append(&sep);
        menu_box.append(&menu_about);
        menu_box.append(&menu_quit);

        let popover = Popover::new();
        popover.set_child(Some(&menu_box));
        popover.set_has_arrow(false);

        let menu = MenuButton::new();
        menu.set_icon_name("open-menu-symbolic");
        menu.set_tooltip_text(Some("Menu"));
        menu.add_css_class("kestrel-toolbar-button");
        menu.set_popover(Some(&popover));

        // ---- Assemble ----
        container.append(&back);
        container.append(&forward);
        container.append(&reload);
        container.append(omnibox.widget());
        container.append(&home);
        container.append(&menu);

        Self {
            container,
            omnibox,
            back,
            forward,
            reload,
            home,
            menu,
            menu_new_tab: menu_button_inner,
            menu_history,
            menu_settings,
            menu_about,
            menu_quit,
        }
    }

    pub fn omnibox(&self) -> &Rc<Omnibox> {
        &self.omnibox
    }

    pub fn on_back<F: Fn() + 'static>(&self, cb: F) {
        self.back.connect_clicked(move |_| cb());
    }

    pub fn on_forward<F: Fn() + 'static>(&self, cb: F) {
        self.forward.connect_clicked(move |_| cb());
    }

    pub fn on_reload<F: Fn() + 'static>(&self, cb: F) {
        self.reload.connect_clicked(move |_| cb());
    }

    pub fn on_home<F: Fn() + 'static>(&self, cb: F) {
        self.home.connect_clicked(move |_| cb());
    }

    pub fn on_new_tab<F: Fn() + 'static>(&self, cb: F) {
        self.menu_new_tab.connect_clicked(move |_| cb());
    }

    pub fn on_history<F: Fn() + 'static>(&self, cb: F) {
        self.menu_history.connect_clicked(move |_| cb());
    }

    pub fn on_settings<F: Fn() + 'static>(&self, cb: F) {
        self.menu_settings.connect_clicked(move |_| cb());
    }

    pub fn on_about<F: Fn() + 'static>(&self, cb: F) {
        self.menu_about.connect_clicked(move |_| cb());
    }

    pub fn on_quit<F: Fn() + 'static>(&self, cb: F) {
        self.menu_quit.connect_clicked(move |_| cb());
    }

    pub fn widget(&self) -> &GtkBox {
        &self.container
    }

    #[allow(dead_code)]
    pub fn menu(&self) -> &MenuButton {
        &self.menu
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper: tạo button với icon symbolic, CSS class chuẩn.
fn make_icon_button(icon_name: &str, tooltip: &str) -> Button {
    let btn = Button::from_icon_name(icon_name);
    btn.set_tooltip_text(Some(tooltip));
    btn.add_css_class("kestrel-toolbar-button");
    btn
}
