//! TabBar — wrapper quanh GTK4 Notebook, có nút "+" thêm tab mới.

use gtk4::prelude::*;
use gtk4::{Button, Label, Notebook, PackType, Widget};

pub struct TabBar {
    notebook: Notebook,
    new_tab_button: Button,
}

impl TabBar {
    pub fn new() -> Self {
        let notebook = Notebook::new();
        notebook.set_vexpand(true);
        notebook.set_hexpand(true);
        notebook.set_scrollable(true);
        notebook.set_show_border(false);
        notebook.add_css_class("kestrel-tabbar");

        // Nút "+" — GTK4 cho phép gắn widget vào tab area qua set_action_widget.
        let new_tab_button = Button::from_icon_name("tab-new-symbolic");
        new_tab_button.set_tooltip_text(Some("New tab"));
        new_tab_button.add_css_class("kestrel-new-tab-button");
        notebook.set_action_widget(&new_tab_button, PackType::End);

        Self {
            notebook,
            new_tab_button,
        }
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

    pub fn on_new_tab<F: Fn() + 'static>(&self, cb: F) {
        self.new_tab_button.connect_clicked(move |_| cb());
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
