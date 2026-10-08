//! TabBar — wrapper quanh GTK4 Notebook.
//!
//! Quản lý tab: thêm, đóng, đổi tên, chuyển tab.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Label, Notebook, Widget};

type SwitchCallback = Box<dyn Fn(usize)>;

pub struct TabBar {
    notebook: Notebook,
    on_switch: Rc<RefCell<Option<SwitchCallback>>>,
}

impl TabBar {
    pub fn new() -> Self {
        let notebook = Notebook::new();
        notebook.set_vexpand(true);
        notebook.set_hexpand(true);
        notebook.set_scrollable(true);
        notebook.set_show_border(false);

        let on_switch: Rc<RefCell<Option<SwitchCallback>>> = Rc::new(RefCell::new(None));
        let cb_holder = on_switch.clone();

        notebook.connect_switch_page(move |_, _, page_num| {
            if let Some(cb) = cb_holder.borrow().as_ref() {
                cb(page_num as usize);
            }
        });

        Self {
            notebook,
            on_switch,
        }
    }

    /// Đặt callback khi user chuyển tab.
    pub fn set_on_switch<F>(&self, cb: F)
    where
        F: Fn(usize) + 'static,
    {
        *self.on_switch.borrow_mut() = Some(Box::new(cb));
    }

    /// Thêm tab mới với content widget và title.
    /// Trả về page number (index của tab vừa thêm).
    pub fn add_tab<W: IsA<Widget>>(&self, content: &W, title: &str) -> u32 {
        let label = Label::new(Some(title));
        label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        label.set_max_width_chars(24);
        label.set_single_line_mode(true);

        let page_num = self.notebook.append_page(content, Some(&label));
        self.notebook.set_current_page(Some(page_num));
        page_num
    }

    /// Đóng tab theo index.
    pub fn close_tab(&self, index: usize) {
        if let Some(child) = self.notebook.nth_page(Some(index as u32)) {
            self.notebook.remove(&child);
        }
    }

    /// Cập nhật title của tab.
    pub fn set_tab_title(&self, index: usize, title: &str) {
        if let Some(child) = self.notebook.nth_page(Some(index as u32)) {
            if let Some(tab_label) = self.notebook.tab_label(&child) {
                if let Some(label) = tab_label.downcast_ref::<Label>() {
                    label.set_text(title);
                }
            }
        }
    }

    /// Chuyển sang tab theo index.
    pub fn set_active(&self, index: usize) {
        self.notebook.set_current_page(Some(index as u32));
    }

    /// Index của tab đang active.
    pub fn active_index(&self) -> usize {
        self.notebook.current_page().unwrap_or(0) as usize
    }

    /// Số tab hiện tại.
    pub fn count(&self) -> usize {
        self.notebook.n_pages() as usize
    }

    /// Trả về GTK widget để append vào container.
    pub fn widget(&self) -> &Notebook {
        &self.notebook
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self::new()
    }
}
