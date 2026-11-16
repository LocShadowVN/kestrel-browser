// src-servo/src/chrome/toolbar.rs
#![forbid(unsafe_code)]
//! Toolbar — Các nút điều hướng SVG + Omnibox (tích hợp sẵn Star, Zoom & Shields)
//! + Thanh tiến trình nội suy 60fps + Find Bar (Ctrl+F kèm bộ đếm kết quả) + Menu.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

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
    is_loading: Rc<Cell<bool>>,
    progress_timer: Rc<RefCell<Option<glib::SourceId>>>,
    home: Button,
    bookmarks_btn: Button,
    adblock: Rc<AdblockButton>,
    progress: ProgressBar,
    find_revealer: Revealer,
    find_entry: Entry,
    find_status: Label,
    find_prev: Button,
    find_next: Button,
    find_close: Button,
    popover: Popover,
    menu_new_tab: Button,
    menu_reopen_tab: Button,
    menu_find: Button,
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
        let menu_reopen_tab = menu_item_with_icon(
            icons::HISTORY_ICON,
            "document-Revert-symbolic",
            "Reopen Closed Tab",
            "Ctrl+Shift+T",
        );
        let menu_find = menu_item_with_icon(
            icons::SEARCH_ICON,
            "edit-find-symbolic",
            "Find in Page",
            "Ctrl+F",
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
        menu_box.append(&menu_reopen_tab);
        menu_box.append(&menu_find);
        let sep1 = Separator::new(Orientation::Horizontal);
        sep1.add_css_class("kestrel-menu-separator");
        menu_box.append(&sep1);
        menu_box.append(&menu_bookmarks);
        menu_box.append(&menu_history);
        menu_box.append(&menu_settings);
        let sep2 = Separator::new(Orientation::Horizontal);
        sep2.add_css_class("kestrel-menu-separator");
        menu_box.append(&sep2);
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

        // --- Thanh Tìm trong trang (Find in Page — Ctrl+F) ---
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

        let find_status = Label::new(None);
        find_status.add_css_class("kestrel-find-status");
        find_status.set_xalign(0.5);

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
            let status_c = find_status.clone();
            find_close.connect_clicked(move |_| {
                rev_c.set_reveal_child(false);
                status_c.set_text("");
                status_c.remove_css_class("no-match");
            });
        }

        {
            let rev_c = find_revealer.clone();
            let entry_c = find_entry.clone();
            let pop_c = popover.clone();
            menu_find.connect_clicked(move |_| {
                pop_c.popdown();
                rev_c.set_reveal_child(true);
                entry_c.grab_focus();
                entry_c.select_region(0, -1);
            });
        }

        find_bar.append(&find_icon);
        find_bar.append(&find_entry);
        find_bar.append(&find_status);
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
            is_loading: Rc::new(Cell::new(false)),
            progress_timer: Rc::new(RefCell::new(None)),
            home,
            bookmarks_btn,
            adblock,
            progress,
            find_revealer,
            find_entry,
            find_status,
            find_prev,
            find_next,
            find_close,
            popover,
            menu_new_tab,
            menu_reopen_tab,
            menu_find,
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
        self.find_status.set_text("");
        self.find_status.remove_css_class("no-match");
    }

    pub fn toggle_find_bar(&self) {
        if self.find_revealer.reveals_child() {
            self.hide_find_bar();
        } else {
            self.show_find_bar();
        }
    }

    pub fn set_find_status(&self, current: usize, total: usize) {
        if self.find_entry.text().trim().is_empty() {
            self.find_status.set_text("");
            self.find_status.remove_css_class("no-match");
            return;
        }
        if total == 0 {
            self.find_status.set_text("0 results");
            self.find_status.add_css_class("no-match");
        } else {
            self.find_status.set_text(&format!("{}/{}", current, total));
            self.find_status.remove_css_class("no-match");
        }
    }

    pub fn on_find<F: Fn(&str, bool) + 'static>(&self, cb: F) {
        let cb = Rc::new(cb);

        {
            let cb_c = cb.clone();
            let status_c = self.find_status.clone();
            self.find_entry.connect_changed(move |entry| {
                let text = entry.text().to_string();
                if text.trim().is_empty() {
                    status_c.set_text("");
                    status_c.remove_css_class("no-match");
                }
                cb_c(&text, false);
            });
        }
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
            let status_c = self.find_status.clone();
            let key_ctrl = EventControllerKey::new();
            key_ctrl.connect_key_pressed(move |_ctrl, keyval, _keycode, state| {
                if keyval == gdk::Key::Escape {
                    rev_c.set_reveal_child(false);
                    status_c.set_text("");
                    status_c.remove_css_class("no-match");
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

        // Hủy timer cũ nếu có
        if let Some(source_id) = self.progress_timer.borrow_mut().take() {
            source_id.remove();
        }

        if loading {
            icons::set_image_svg(&self.reload_icon, icons::STOP, "process-stop-symbolic", 16);
            self.reload.set_tooltip_text(Some("Stop / Reload"));
            self.progress.set_fraction(0.15);
            self.progress.set_visible(true);

            let progress_c = self.progress.clone();
            let is_loading_c = self.is_loading.clone();
            let timer_slot = self.progress_timer.clone();
            let id = glib::timeout_add_local(Duration::from_millis(50), move || {
                if !is_loading_c.get() {
                    *timer_slot.borrow_mut() = None;
                    return glib::ControlFlow::Break;
                }
                let cur = progress_c.fraction();
                let remaining = (0.92 - cur).max(0.0);
                progress_c.set_fraction(cur + remaining * 0.08);
                glib::ControlFlow::Continue
            });
            *self.progress_timer.borrow_mut() = Some(id);
        } else {
            icons::set_image_svg(
                &self.reload_icon,
                icons::RELOAD,
                "view-refresh-symbolic",
                16,
            );
            self.reload.set_tooltip_text(Some("Reload (Ctrl+R)"));
            self.progress.set_fraction(1.0);

            let progress_c = self.progress.clone();
            let is_loading_c = self.is_loading.clone();
            let timer_slot = self.progress_timer.clone();
            let id = glib::timeout_add_local(Duration::from_millis(140), move || {
                if !is_loading_c.get() {
                    progress_c.set_visible(false);
                    progress_c.set_fraction(0.0);
                }
                *timer_slot.borrow_mut() = None;
                glib::ControlFlow::Break
            });
            *self.progress_timer.borrow_mut() = Some(id);
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
    pub fn on_reopen_closed_tab<F: Fn() + 'static>(&self, cb: F) {
        let pop = self.popover.clone();
        self.menu_reopen_tab.connect_clicked(move |_| {
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
