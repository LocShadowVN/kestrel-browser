//! Toolbar — navigation buttons + omnibox.

use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, MenuButton, Orientation};

use crate::chrome::omnibox::Omnibox;

pub struct Toolbar {
    container: GtkBox,
    omnibox: Rc<Omnibox>,
    back: Button,
    forward: Button,
    reload: Button,
    home: Button,
    menu: MenuButton,
}

impl Toolbar {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 4);
        container.set_margin_top(4);
        container.set_margin_bottom(4);
        container.set_margin_start(4);
        container.set_margin_end(4);

        let back = Button::from_icon_name("go-previous-symbolic");
        back.set_tooltip_text(Some("Back"));

        let forward = Button::from_icon_name("go-next-symbolic");
        forward.set_tooltip_text(Some("Forward"));

        let reload = Button::from_icon_name("view-refresh-symbolic");
        reload.set_tooltip_text(Some("Reload"));

        let omnibox = Rc::new(Omnibox::new());

        let home = Button::from_icon_name("go-home-symbolic");
        home.set_tooltip_text(Some("Home"));

        let menu = MenuButton::new();
        menu.set_icon_name("open-menu-symbolic");

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
