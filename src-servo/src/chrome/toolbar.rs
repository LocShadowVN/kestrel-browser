//! Toolbar — navigation buttons + omnibox + hamburger menu.

use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, MenuButton, Orientation, Popover, Separator};

use crate::chrome::omnibox::Omnibox;

pub struct Toolbar {
    container: GtkBox,
    omnibox: Rc<Omnibox>,
    back: Button,
    forward: Button,
    reload: Button,
    home: Button,
    menu_new_tab: Button,
    menu_history: Button,
    menu_settings: Button,
    menu_about: Button,
    menu_quit: Button,
    #[allow(dead_code)]
    menu: MenuButton,
}

impl Toolbar {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 0);
        container.add_css_class("kestrel-toolbar");
        container.set_spacing(4);

        let back = make_icon_button("go-previous-symbolic", "Back");
        let forward = make_icon_button("go-next-symbolic", "Forward");
        let reload = make_icon_button("view-refresh-symbolic", "Reload");

        let omnibox = Rc::new(Omnibox::new());

        let home = make_icon_button("go-home-symbolic", "Home");

        // Menu items
        let menu_new_tab = menu_item("New Tab");
        let menu_history = menu_item("History");
        let menu_settings = menu_item("Settings");
        let menu_about = menu_item("About");
        let menu_quit = menu_item("Quit");

        let menu_box = GtkBox::new(Orientation::Vertical, 0);
        menu_box.add_css_class("kestrel-menu");
        menu_box.append(&menu_new_tab);
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
            menu_new_tab,
            menu_history,
            menu_settings,
            menu_about,
            menu_quit,
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
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

fn make_icon_button(icon_name: &str, tooltip: &str) -> Button {
    let btn = Button::from_icon_name(icon_name);
    btn.set_tooltip_text(Some(tooltip));
    btn.add_css_class("kestrel-toolbar-button");
    btn
}

fn menu_item(label: &str) -> Button {
    let btn = Button::with_label(label);
    btn.add_css_class("kestrel-menu-item");
    btn
}
