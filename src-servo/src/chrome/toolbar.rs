// src-servo/src/chrome/toolbar.rs
//! Toolbar — Các nút điều hướng SVG + Omnibox (tích hợp sẵn Star, Zoom & Shields) + Find Bar (Ctrl+F) + Menu.

use std::cell::Cell;
use std::rc::Rc;

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Entry, EventControllerKey, Image, Label, MenuButton, Orientation,
    Popover, ProgressBar, Revealer, RevealerTransitionType, Separator,
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
    find_revealer: Revealer,
    find_entry: Entry,
    find_prev: Button,
    find_next: Button,
    find_close: Button,
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
            17,
            "Back (Alt+Left)",
        );
        back.add_css_class("kestrel-toolbar-button");

        let forward = icons::svg_button(
            icons::ARROW_RIGHT,
            "go-next-symbolic",
            17,
            "Forward (Alt+Right)",
        );
        forward.add_css_class("kestrel-toolbar-button");

        let reload_icon = icons::svg_image(icons::RELOAD, "view-refresh-symbolic", 16);
        let reload = Button::new();
        reload.set_child(Some(&reload_icon));
        reload.set_tooltip_text(Some("Reload (Ctrl+R)"));
        reload.add_css_class("kestrel-toolbar-button");

        let home = icons::svg_button(icons::HOME, "go-home-symbolic", 16, "Home (Alt+Home)");
        home.add_css_class("kestrel-toolbar-button");

        let adblock = Rc::new(AdblockButton::new(
            AdblockLevel::Standard,
            on_adblock_change,
        ));

        let omnibox = Rc::new(Omnibox::new(&adblock));

        let bookmarks_btn = icons::svg_button(
            icons::BOOKMARKS_ICON,
            "user-bookmarks-symbolic",
            16,
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
        let menu_img = icons::svg_image(icons::MENU, "open-menu-symbolic", 17);
        menu.set_child(Some(&menu_img));
        menu.set_tooltip_text(Some("Menu"));
        menu.add_css_class("kestrel-toolbar-button");
        menu.set_popover(Some(&popover));

        container.append(&back);
        container.append(&forward);
        container.append(&reload);
        container.append(&home);
        container.append(omnibox.widget());
        container.append(&bookmarks_btn);
        container.append(&menu);

        let progress = ProgressBar::new();
        progress.add_css_class("kestrel-progress");
        progress.set_fraction(0.0);
        progress.set_visible(false);

        // --- Thanh Tìm trong trang (Find in Page — Ctrl+F) với Icon SVG đồng bộ ---
        let find_revealer = Revealer::builder()
            .transition_type(RevealerTransitionType::SlideDown)
            .transition_duration(120)
            .reveal_child(false)
            .build();

        let find_bar = GtkBox::new(Orientation::Horizontal, 6);
        find_bar.add_css_class("kestrel-find-bar");

        let find_icon = icons::svg_image(icons::SEARCH_ICON, "system-search-symbolic", 14);
        let find_entry = Entry::builder()
            .placeholder_text("Find in page...")
            .width_chars(28)
            .build();
        find_entry.add_css_class("kestrel-find-entry");

        let find_prev = icons::svg_button(
            icons::CHEVRON_UP,
            "go-up-symbolic",
            15,
            "Previous match (Shift+Enter)",
        );
        find_prev.add_css_class("kestrel-toolbar-button");

        let find_next = icons::svg_button(
            icons::CHEVRON_DOWN,
            "go-down-symbolic",
            15,
            "Next match (Enter)",
        );
        find_next.add_css_class("kestrel-toolbar-button");

        let find_close = icons::svg_button(
            icons::CLOSE,
            "window-close-symbolic",
            13,
            "Close find bar (Esc)",
        );
        find_close.add_css_class("kestrel-toolbar-button");

        {
            let rev_c = find_revealer.clone();
            find_close.connect_clicked(move |_| {
                rev_c.set_reveal_child(false);
            });
        }

        find_bar.append(&find_icon);
        find_bar.append(&find_entry);
        find_bar.append(&find_prev);
        find_bar.append(&find_next);
        find_bar.append(&find_close);
        find_revealer.set_child(Some(&find_bar));

        wrapper.append(&container);
        wrapper.append(&progress);
        wrapper.append(&find_revealer);

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
            find_revealer,
            find_entry,
            find_prev,
            find_next,
            find_close,
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

    pub fn set_nav_state(&self, can_go_back: bool, can_go_forward: bool) {
        self.back.set_sensitive(can_go_back);
        self.forward.set_sensitive(can_go_forward);
    }

    pub fn show_find_bar(&self) {
        self.find_revealer.set_reveal_child(true);
        self.find_entry.grab_focus();
        self.find_entry.select_region(0, -1);
    }

    pub fn hide_find_bar(&self) {
        self.find_revealer.set_reveal_child(false);
    }

    pub fn toggle_find_bar(&self) {
        if self.find_revealer.reveals_child() {
            self.hide_find_bar();
        } else {
            self.show_find_bar();
        }
    }

    pub fn on_find<F: Fn(&str, bool) + 'static>(&self, cb: F) {
        let cb = Rc::new(cb);

        {
            let cb_c = cb.clone();
            self.find_entry.connect_activate(move |entry| {
                let text = entry.text().to_string();
                cb_c(&text, false);
            });
        }
        {
            let cb_c = cb.clone();
            let entry_c = self.find_entry.clone();
            let rev_c = self.find_revealer.clone();
            let key_ctrl = EventControllerKey::new();
            key_ctrl.connect_key_pressed(move |_ctrl, keyval, _keycode, state| {
                if keyval == gdk::Key::Escape {
                    rev_c.set_reveal_child(false);
                    cb_c("", false);
                    return glib::Propagation::Stop;
                }
                if (keyval == gdk::Key::Return || keyval == gdk::Key::KP_Enter)
                    && state.contains(gdk::ModifierType::SHIFT_MASK)
                {
                    let text = entry_c.text().to_string();
                    cb_c(&text, true);
                    return glib::Propagation::Stop;
                }
                glib::Propagation::Proceed
            });
            self.find_entry.add_controller(key_ctrl);
        }
        {
            let cb_c = cb.clone();
            let entry_c = self.find_entry.clone();
            self.find_next.connect_clicked(move |_| {
                let text = entry_c.text().to_string();
                cb_c(&text, false);
            });
        }
        {
            let cb_c = cb.clone();
            let entry_c = self.find_entry.clone();
            self.find_prev.connect_clicked(move |_| {
                let text = entry_c.text().to_string();
                cb_c(&text, true);
            });
        }
        {
            let cb_c = cb;
            self.find_close.connect_clicked(move |_| {
                cb_c("", false);
            });
        }
    }

    pub fn set_loading(&self, loading: bool) {
        if self.is_loading.get() == loading {
            return;
        }
        self.is_loading.set(loading);
        if loading {
            icons::set_image_svg(&self.reload_icon, icons::STOP, "process-stop-symbolic", 16);
            self.reload.set_tooltip_text(Some("Stop loading"));
            self.progress.set_fraction(0.35);
            self.progress.set_visible(true);
        } else {
            icons::set_image_svg(
                &self.reload_icon,
                icons::RELOAD,
                "view-refresh-symbolic",
                16,
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
