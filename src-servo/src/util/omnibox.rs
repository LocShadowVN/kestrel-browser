//! Omnibox — URL + search bar.
//!
//! Wrapper quanh GTK4 Entry để tách logic khỏi toolbar.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::Entry;

type SubmitCallback = Box<dyn Fn(&str)>;

pub struct Omnibox {
    entry: Entry,
    on_submit: Rc<RefCell<Option<SubmitCallback>>>,
}

impl Omnibox {
    pub fn new() -> Self {
        let entry = Entry::new();
        entry.set_hexpand(true);
        entry.set_placeholder_text(Some("Search or enter URL"));

        let on_submit: Rc<RefCell<Option<SubmitCallback>>> = Rc::new(RefCell::new(None));
        let cb_holder = on_submit.clone();

        entry.connect_activate(move |entry| {
            let text = entry.text().to_string();
            if let Some(cb) = cb_holder.borrow().as_ref() {
                cb(&text);
            }
        });

        Self { entry, on_submit }
    }

    /// Đặt callback khi user nhấn Enter.
    pub fn set_on_submit<F>(&self, cb: F)
    where
        F: Fn(&str) + 'static,
    {
        *self.on_submit.borrow_mut() = Some(Box::new(cb));
    }

    /// Cập nhật text (khi navigation xong, sync URL hiện tại).
    pub fn set_text(&self, text: &str) {
        self.entry.set_text(text);
    }

    /// Lấy text hiện tại.
    pub fn text(&self) -> String {
        self.entry.text().to_string()
    }

    /// Trả về GTK widget để append vào container.
    pub fn widget(&self) -> &Entry {
        &self.entry
    }

    /// Focus vào omnibox (dùng cho Ctrl+L).
    pub fn focus(&self) {
        self.entry.grab_focus();
        self.entry.select_region(0, -1);
    }
}

impl Default for Omnibox {
    fn default() -> Self {
        Self::new()
    }
}
