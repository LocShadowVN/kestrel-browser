//! Omnibox — Thanh địa chỉ tích hợp:
//! [ Icon bảo mật ] [ Ô nhập địa chỉ / tìm kiếm ] [ Ngôi sao Bookmark ] [ | ] [ Khiên Shields + Số đếm ]

use std::cell::Cell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Entry, Image, Orientation, Separator};

use crate::chrome::adblock_button::AdblockButton;
use crate::chrome::icons;

pub struct Omnibox {
    container: GtkBox,
    security_icon: Image,
    security_state: Cell<u8>,
    entry: Entry,
    star_btn: Button,
    star_icon: Image,
    bookmarked: Cell<bool>,
}

impl Omnibox {
    pub fn new(adblock: &Rc<AdblockButton>) -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 4);
        container.add_css_class("kestrel-omnibox-box");
        container.set_hexpand(true);

        let security_icon =
            icons::svg_image(icons::KESTREL_BADGE, "web-browser-symbolic", 14);
        security_icon.add_css_class("kestrel-omnibox-security");
        security_icon.set_tooltip_text(Some("Kestrel Internal Page"));

        let entry = Entry::builder()
            .placeholder_text("Search the web or enter address...")
            .hexpand(true)
            .build();
        entry.add_css_class("kestrel-omnibox-entry");

        let star_icon = icons::svg_image(icons::STAR_OUTLINE, "non-starred-symbolic", 15);
        let star_btn = Button::new();
        star_btn.set_child(Some(&star_icon));
        star_btn.add_css_class("kestrel-omnibox-action");
        star_btn.set_tooltip_text(Some("Bookmark this page (Ctrl+D)"));

        let divider = Separator::new(Orientation::Vertical);
        divider.add_css_class("kestrel-omnibox-divider");

        container.append(&security_icon);
        container.append(&entry);
        container.append(&star_btn);
        container.append(&divider);
        container.append(adblock.widget());

        Self {
            container,
            security_icon,
            security_state: Cell::new(0),
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
        if self.entry.text().as_str() != text {
            self.entry.set_text(text);
        }
        self.update_security_indicator(text);
    }

    #[allow(dead_code)]
    pub fn text(&self) -> String {
        self.entry.text().to_string()
    }

    pub fn set_bookmarked(&self, is_bookmarked: bool) {
        if self.bookmarked.get() == is_bookmarked {
            return;
        }
        self.bookmarked.set(is_bookmarked);
        if is_bookmarked {
            self.star_btn.add_css_class("bookmarked");
            icons::set_image_svg(&self.star_icon, icons::STAR_FILLED, "starred-symbolic", 15);
            self.star_btn
                .set_tooltip_text(Some("Remove bookmark (Ctrl+D)"));
        } else {
            self.star_btn.remove_css_class("bookmarked");
            icons::set_image_svg(
                &self.star_icon,
                icons::STAR_OUTLINE,
                "non-starred-symbolic",
                15,
            );
            self.star_btn
                .set_tooltip_text(Some("Bookmark this page (Ctrl+D)"));
        }
    }

    #[allow(dead_code)]
    pub fn is_bookmarked(&self) -> bool {
        self.bookmarked.get()
    }

    fn update_security_indicator(&self, url: &str) {
        let u = url.trim().to_ascii_lowercase();
        let new_state = if u.starts_with("https://") {
            1
        } else if u.starts_with("http://") {
            2
        } else if u.starts_with("kestrel://") || u == "kestrel:" {
            3
        } else {
            4
        };

        if self.security_state.get() == new_state {
            return;
        }
        self.security_state.set(new_state);

        match new_state {
            1 => {
                icons::set_image_svg(
                    &self.security_icon,
                    icons::LOCK_SECURE,
                    "channel-secure-symbolic",
                    14,
                );
                self.security_icon
                    .set_tooltip_text(Some("Connection is secure (HTTPS)"));
            }
            2 => {
                icons::set_image_svg(
                    &self.security_icon,
                    icons::INFO_INSECURE,
                    "dialog-warning-symbolic",
                    14,
                );
                self.security_icon
                    .set_tooltip_text(Some("Connection is not encrypted (HTTP)"));
            }
            3 => {
                icons::set_image_svg(
                    &self.security_icon,
                    icons::KESTREL_BADGE,
                    "web-browser-symbolic",
                    14,
                );
                self.security_icon
                    .set_tooltip_text(Some("Kestrel Internal Page"));
            }
            _ => {
                icons::set_image_svg(
                    &self.security_icon,
                    icons::SEARCH_ICON,
                    "system-search-symbolic",
                    14,
                );
                self.security_icon
                    .set_tooltip_text(Some("Search or enter address"));
            }
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
