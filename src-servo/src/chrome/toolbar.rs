//! Toolbar — Các nút điều hướng SVG + Omnibox + Kestrel Shields + Bookmarks + Menu + ProgressBar.

use std::cell::Cell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Image, Label, MenuButton, Orientation, Popover, ProgressBar, Separator,
};

use crate::chrome::adblock_button::{AdblockButton, AdblockLevel};
use crate::chrome::icons;
use crate::chrome::omnibox::Omnibox;

pub struct Toolbar {
    wrapper: GtkBox,
    omnibox: Rc<Omnibox>,
    back: Button,
    forward: Button,
    reload: Button,
    reload_icon: Image,
    is_loading: Cell<bool>,
    home: Button,
    bookmarks_btn: Button,
    adblock: Rc<AdblockButton>,
    progress: ProgressBar,
    popover: Popover,
    menu_new_tab: Button,
    menu_history: Button,
    menu_settings: Button,
    menu_bookmarks: Button,
    menu_about: Button,
    menu_quit: Button,
    #[allow(dead_code)]
    menu: MenuButton,
}

impl Toolbar {
    pub fn new<F>(on_adblock_change: F) -> Self
    where
        F: Fn(u8) + 'static,
    {
        let wrapper = GtkBox::new(Orientation::Vertical, 0);

        let container = GtkBox::new(Orientation::Horizontal, 2);
        container.add_css_class("kestrel-toolbar");

        let back = icons::svg_button(
            icons::ARROW_LEFT,
            "go-previous-symbolic",
            18,
            "Back (Alt+Left)",
        );
        back.add_css_class("kestrel-toolbar-button");

        let forward = icons::svg_button(
            icons::ARROW_RIGHT,
            "go-next-symbolic",
            18,
            "Forward (Alt+Right)",
        );
        forward.add_css_class("kestrel-toolbar-button");

        let reload_icon = icons::svg_image(icons::RELOAD, "view-refresh-symbolic", 17);
        let reload = Button::new();
        reload.set_child(Some(&reload_icon));
        reload.set_tooltip_text(Some("Reload (Ctrl+R)"));
        reload.add_css_class("kestrel-toolbar-button");

        let home = icons::svg_button(icons::HOME, "go-home-symbolic", 17, "Home");
        home.add_css_class("kestrel-toolbar-button");

        let omnibox = Rc::new(Omnibox::new());

        let adblock = Rc::new(AdblockButton::new(
            AdblockLevel::Standard,
            on_adblock_change,
        ));

        let bookmarks_btn = icons::svg_button(
            icons::BOOKMARKS_ICON,
            "user-bookmarks-symbolic",
            17,
            "Bookmarks (Ctrl+B)",
        );
        bookmarks_btn.add_css_class("kestrel-toolbar-button");

        let menu_new_tab = menu_item_with_icon(
            icons::PLUS,
            "tab-new-symbolic",
            "New Tab",
            "Ctrl+T",
        );
        let menu_bookmarks = menu_item_with_icon(
            icons::BOOKMARKS_ICON,
            "user-bookmarks-symbolic",
            "Bookmarks",
            "Ctrl+B",
        );
        let menu_history = menu_item_with_icon(
            icons::HISTORY_ICON,
            "document-open-recent-symbolic",
            "History",
            "Ctrl+H",
        );
        let menu_settings = menu_item_with_icon(
            icons::SETTINGS_ICON,
            "emblem-system-symbolic",
            "Settings",
            "Ctrl+,",
        );
        let menu_about = menu_item_with_icon(
            icons::INFO_ICON,
            "help-about-symbolic",
            "About Kestrel",
            "",
        );
        let menu_quit = menu_item_with_icon(
            icons::QUIT_ICON,
            "application-exit-symbolic",
            "Quit",
            "Ctrl+Q",
        );

        let menu_box = GtkBox::new(Orientation::Vertical, 2);
        menu_box.add_css_class("kestrel-menu");
        menu_box.append(&menu_new_tab);
        menu_box.append(&menu_bookmarks);
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
        let menu_img = icons::svg_image(icons::MENU, "open-menu-symbolic", 18);
        menu.set_child(Some(&menu_img));
        menu.set_tooltip_text(Some("Menu"));
        menu.add_css_class("kestrel-toolbar-button");
        menu.set_popover(Some(&popover));

        container.append(&back);
        container.append(&forward);
        container.append(&reload);
        container.append(&home);
        container.append(omnibox.widget());
        container.append(adblock.widget());
        container.append(&bookmarks_btn);
        container.append(&menu);

        let progress = ProgressBar::new();
        progress.add_css_class("kestrel-progress");
        progress.set_fraction(0.0);
        progress.set_visible(false);

        wrapper.append(&container);
        wrapper.append(&progress);

        Self {
            wrapper,
            omnibox,
            back,
            forward,
            reload,
            reload_icon,
            is_loading: Cell::new(false),
            home,
            bookmarks_btn,
            adblock,
            progress,
            popover,
            menu_new_tab,
            menu_history,
            menu_settings,
            menu_bookmarks,
            menu_about,
            menu_quit,
            menu,
        }
    }

    pub fn omnibox(&self) -> &Rc<Omnibox> {
        &self.omnibox
    }

    pub fn adblock(&self) -> &Rc<AdblockButton> {
        &self.adblock
    }

    pub fn set_adblock_level(&self, level: u8) {
        self.adblock.set_level(level);
    }

    pub fn set_loading(&self, loading: bool) {
        self.is_loading.set(loading);
        if loading {
            icons::set_image_svg(&self.reload_icon, icons::STOP, "process-stop-symbolic", 17);
            self.reload.set_tooltip_text(Some("Stop loading"));
            self.progress.set_fraction(0.35);
            self.progress.set_visible(true);
        } else {
            icons::set_image_svg(
                &self.reload_icon,
                icons::RELOAD,
                "view-refresh-symbolic",
                17,
            );
            self.reload.set_tooltip_text(Some("Reload (Ctrl+R)"));
            self.progress.set_fraction(1.0);
            self.progress.set_visible(false);
        }
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
    pub fn on_bookmark<F: Fn() + 'static>(&self, cb: F) {
        self.omnibox.on_bookmark_toggle(cb);
    }
    pub fn on_bookmarks_button<F: Fn() + 'static>(&self, cb: F) {
        self.bookmarks_btn.connect_clicked(move |_| cb());
    }
    pub fn on_new_tab<F: Fn() + 'static>(&self, cb: F) {
        let pop = self.popover.clone();
        self.menu_new_tab.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
    }
    pub fn on_history<F: Fn() + 'static>(&self, cb: F) {
        let pop = self.popover.clone();
        self.menu_history.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
    }
    pub fn on_settings<F: Fn() + 'static>(&self, cb: F) {
        let pop = self.popover.clone();
        self.menu_settings.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
    }
    pub fn on_bookmarks<F: Fn() + 'static>(&self, cb: F) {
        let pop = self.popover.clone();
        self.menu_bookmarks.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
    }
    pub fn on_about<F: Fn() + 'static>(&self, cb: F) {
        let pop = self.popover.clone();
        self.menu_about.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
    }
    pub fn on_quit<F: Fn() + 'static>(&self, cb: F) {
        let pop = self.popover.clone();
        self.menu_quit.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
    }

    pub fn widget(&self) -> &GtkBox {
        &self.wrapper
    }
}

fn menu_item_with_icon(
    svg: &'static str,
    fallback: &str,
    label_text: &str,
    shortcut: &str,
) -> Button {
    let btn = Button::new();
    btn.add_css_class("kestrel-menu-item");

    let row = GtkBox::new(Orientation::Horizontal, 10);
    let icon = icons::svg_image(svg, fallback, 15);
    let lbl = Label::new(Some(label_text));
    lbl.set_xalign(0.0);
    lbl.set_hexpand(true);

    row.append(&icon);
    row.append(&lbl);

    if !shortcut.is_empty() {
        let sc = Label::new(Some(shortcut));
        sc.add_css_class("kestrel-menu-shortcut");
        row.append(&sc);
    }

    btn.set_child(Some(&row));
    btn
}
