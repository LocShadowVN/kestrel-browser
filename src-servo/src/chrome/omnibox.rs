//! Omnibox — wrapper quanh GTK4 Entry.

use gtk4::prelude::*;
use gtk4::Entry;

pub struct Omnibox {
    entry: Entry,
}

impl Omnibox {
    pub fn new() -> Self {
        let entry = Entry::builder()
            .placeholder_text("Tìm kiếm hoặc nhập URL...")
            .hexpand(true)
            .build();
        entry.add_css_class("kestrel-omnibox");

        Self { entry }
    }

    pub fn on_activate<F: Fn(&str) + 'static>(&self, cb: F) {
        self.entry.connect_activate(move |entry| {
            let text = entry.text().to_string();
            cb(&text);
        });
    }

    pub fn set_text(&self, text: &str) {
        self.entry.set_text(text);
    }

    pub fn text(&self) -> String {
        self.entry.text().to_string()
    }

    pub fn focus_and_select(&self) {
        self.entry.grab_focus();
        self.entry.select_region(0, -1);
    }

    pub fn widget(&self) -> &Entry {
        &self.entry
    }
}

impl Default for Omnibox {
    fn default() -> Self {
        Self::new()
    }
}
