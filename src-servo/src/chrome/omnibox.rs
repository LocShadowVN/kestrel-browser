//! Omnibox — Thanh địa chỉ hiện đại kiểu Brave:
//! [ Icon bảo mật (HTTPS / Kestrel / Search) ] [ Entry nhập URL ] [ Nút sao Bookmark ]

use std::cell::Cell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Entry, Image, Label, Orientation};

use crate::chrome::icons;

pub struct Omnibox {
    container: GtkBox,
    security_icon: Image,
    scheme_badge: Label,
    entry: Entry,
    star_btn: Button,
    star_icon: Image,
    bookmarked: Cell<bool>,
}

impl Omnibox {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 4);
        container.add_css_class("kestrel-omnibox-box");
        container.set_hexpand(true);

        let security_icon =
            icons::svg_image(icons::KESTREL_BADGE, "channel-secure-symbolic", 15);
        security_icon.add_css_class("kestrel-omnibox-security");
        security_icon.set_tooltip_text(Some("Kestrel Internal Page"));

        let scheme_badge = Label::new(Some("Kestrel"));
        scheme_badge.add_css_class("kestrel-omnibox-scheme-badge");

        let entry = Entry::builder()
            .placeholder_text("Search with Brave or enter address...")
            .hexpand(true)
            .build();
        entry.add_css_class("kestrel-omnibox-entry");

        let star_icon = icons::svg_image(icons::STAR_OUTLINE, "non-starred-symbolic", 16);
        let star_btn = Button::new();
        star_btn.set_child(Some(&star_icon));
        star_btn.add_css_class("kestrel-omnibox-star");
        star_btn.set_tooltip_text(Some("Bookmark this page (Ctrl+D)"));

        container.append(&security_icon);
        container.append(&scheme_badge);
        container.append(&entry);
        container.append(&star_btn);

        Self {
            container,
            security_icon,
            scheme_badge,
            entry,
            star_btn,
            star_icon,
            bookmarked: Cell::new(false),
        }
    }

    pub fn on_activate<F: Fn(&str) + 'static>(&self, cb: F) {
        self.entry.connect_activate(move |entry| {
            let text = entry.text().to_string();
            cb(&text);
        });
    }

    pub fn on_bookmark_toggle<F: Fn() + 'static>(&self, cb: F) {
        self.star_btn.connect_clicked(move |_| cb());
    }

    pub fn set_text(&self, text: &str) {
        self.entry.set_text(text);
        self.update_security_indicator(text);
    }

    pub fn text(&self) -> String {
        self.entry.text().to_string()
    }

    pub fn set_bookmarked(&self, is_bookmarked: bool) {
        self.bookmarked.set(is_bookmarked);
        if is_bookmarked {
            icons::set_image_svg(&self.star_icon, icons::STAR_FILLED, "starred-symbolic", 16);
            self.star_btn
                .set_tooltip_text(Some("Remove bookmark (Ctrl+D)"));
        } else {
            icons::set_image_svg(
                &self.star_icon,
                icons::STAR_OUTLINE,
                "non-starred-symbolic",
                16,
            );
            self.star_btn
                .set_tooltip_text(Some("Bookmark this page (Ctrl+D)"));
        }
    }

    pub fn is_bookmarked(&self) -> bool {
        self.bookmarked.get()
    }

    fn update_security_indicator(&self, url: &str) {
        let u = url.trim();
        if u.starts_with("https://") {
            icons::set_image_svg(
                &self.security_icon,
                icons::LOCK_SECURE,
                "channel-secure-symbolic",
                15,
            );
            self.security_icon
                .set_tooltip_text(Some("Connection is secure (HTTPS)"));
            self.scheme_badge.set_visible(false);
        } else if u.starts_with("http://") {
            icons::set_image_svg(
                &self.security_icon,
                icons::INFO_INSECURE,
                "dialog-warning-symbolic",
                15,
            );
            self.security_icon
                .set_tooltip_text(Some("Connection is not encrypted (HTTP)"));
            self.scheme_badge.set_text("Not Secure");
            self.scheme_badge.set_visible(true);
        } else if u.starts_with("kestrel://") || u == "kestrel:" {
            icons::set_image_svg(
                &self.security_icon,
                icons::KESTREL_BADGE,
                "web-browser-symbolic",
                16,
            );
            self.security_icon
                .set_tooltip_text(Some("Kestrel Internal Page"));
            self.scheme_badge.set_text("Kestrel");
            self.scheme_badge.set_visible(true);
        } else {
            icons::set_image_svg(
                &self.security_icon,
                icons::SEARCH_ICON,
                "system-search-symbolic",
                15,
            );
            self.security_icon
                .set_tooltip_text(Some("Search or enter URL"));
            self.scheme_badge.set_visible(false);
        }
    }

    pub fn focus_and_select(&self) {
        self.entry.grab_focus();
        self.entry.select_region(0, -1);
    }

    pub fn widget(&self) -> &GtkBox {
        &self.container
    }
}

impl Default for Omnibox {
    fn default() -> Self {
        Self::new()
    }
}
