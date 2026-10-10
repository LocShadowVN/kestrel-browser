//! TabBar — wrapper quanh GTK4 Notebook.

use gtk4::prelude::*;
use gtk4::{Label, Notebook, Widget};

pub struct TabBar {
    notebook: Notebook,
}

impl TabBar {
    pub fn new() -> Self {
        let notebook = Notebook::new();
        notebook.set_vexpand(true);
        notebook.set_hexpand(true);
        notebook.set_scrollable(true);
        notebook.set_show_border(false);
        notebook.add_css_class("kestrel-tabbar");

        Self { notebook }
    }

    pub fn add_tab<W: IsA<Widget>>(&self, content: &W, title: &str) -> u32 {
        let label = Label::new(Some(title));
        label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        label.set_width_chars(1);
        label.set_max_width_chars(20);
        label.set_single_line_mode(true);

        let page_num = self.notebook.append_page(content, Some(&label));
        self.notebook.set_current_page(Some(page_num));
        page_num
    }

    pub fn close_tab(&self, index: usize) {
        let page_num = index as u32;
        if self.notebook.nth_page(Some(page_num)).is_some() {
            self.notebook.remove_page(Some(page_num));
        }
    }

    pub fn set_tab_title(&self, page_num: u32, title: &str) {
        if let Some(child) = self.notebook.nth_page(Some(page_num)) {
            if let Some(tab_label) = self.notebook.tab_label(&child) {
                if let Some(label) = tab_label.downcast_ref::<Label>() {
                    label.set_text(title);
                }
            }
        }
    }

    pub fn set_active(&self, index: usize) {
        self.notebook.set_current_page(Some(index as u32));
    }

    pub fn active_index(&self) -> usize {
        self.notebook.current_page().unwrap_or(0) as usize
    }

    pub fn count(&self) -> usize {
        self.notebook.n_pages() as usize
    }

    pub fn on_switch<F: Fn(u32) + 'static>(&self, cb: F) {
        self.notebook.connect_switch_page(move |_, _, page_num| {
            cb(page_num);
        });
    }

    pub fn widget(&self) -> &Notebook {
        &self.notebook
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self::new()
    }
}
