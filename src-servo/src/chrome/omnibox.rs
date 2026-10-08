//! Omnibox — wrapper quanh GTK4 Entry.
//!
//! Phase 1: chỉ hiển thị URL, nhận Enter để navigate.
//! Phase 2: thêm autocomplete từ history/bookmarks.

use gtk4::prelude::*;
use gtk4::Entry;

pub struct Omnibox {
    entry: Entry,
}

impl Omnibox {
    pub fn new() -> Self {
        let entry = Entry::builder()
            .placeholder_text("Search or enter URL")
            .hexpand(true)
            .build();

        Self { entry }
    }

    /// Callback khi user nhấn Enter.
    pub fn on_activate<F: Fn(&str) + 'static>(&self, cb: F) {
        self.entry.connect_activate(move |entry| {
            let text = entry.text().to_string();
            cb(&text);
        });
    }

    /// Cập nhật text (khi navigation xong, sync URL hiện tại).
    pub fn set_text(&self, text: &str) {
        self.entry.set_text(text);
    }

    /// Text hiện tại.
    pub fn text(&self) -> String {
        self.entry.text().to_string()
    }

    /// Focus + select all (Ctrl+L).
    pub fn focus_and_select(&self) {
        self.entry.grab_focus();
        self.entry.select_region(0, -1);
    }

    /// GTK widget để append vào container.
    pub fn widget(&self) -> &Entry {
        &self.entry
    }
}

impl Default for Omnibox {
    fn default() -> Self {
        Self::new()
    }
}
