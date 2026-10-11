//! TabBar — wrapper quanh GTK4 Notebook, custom tab widget giống Brave.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Button, Notebook, PackType, Widget};

use crate::chrome::tab_widget::TabWidget;

struct TabMeta {
    widget: Rc<TabWidget>,
    page_num: u32,
}

pub struct TabBar {
    notebook: Notebook,
    new_tab_button: Button,
    tabs: RefCell<Vec<Rc<TabMeta>>>,
}

impl TabBar {
    pub fn new() -> Self {
        let notebook = Notebook::new();
        notebook.set_vexpand(true);
        notebook.set_hexpand(true);
        notebook.set_scrollable(true);
        notebook.set_show_border(false);
        notebook.add_css_class("kestrel-tabbar");

        let new_tab_button = Button::from_icon_name("tab-new-symbolic");
        new_tab_button.set_tooltip_text(Some("New tab"));
        new_tab_button.add_css_class("kestrel-new-tab-button");
        notebook.set_action_widget(&new_tab_button, PackType::End);

        Self {
            notebook,
            new_tab_button,
            tabs: RefCell::new(Vec::new()),
        }
    }

    pub fn add_tab<W, F>(&self, content: &W, title: &str, on_close: F) -> u32
    where
        W: IsA<Widget>,
        F: Fn() + 'static,
    {
        let tab_widget = Rc::new(TabWidget::new(title, on_close));
        let page_num = self
            .notebook
            .append_page(content, Some(tab_widget.widget()));

        self.tabs.borrow_mut().push(Rc::new(TabMeta {
            widget: tab_widget,
            page_num,
        }));

        self.notebook.set_current_page(Some(page_num));
        page_num
    }

    pub fn close_tab(&self, page_num: u32) {
        if self.notebook.nth_page(Some(page_num)).is_some() {
            self.notebook.remove_page(Some(page_num));
        }
        self.tabs.borrow_mut().retain(|t| t.page_num != page_num);
    }

    pub fn set_tab_title(&self, page_num: u32, title: &str) {
        if let Some(meta) = self.tabs.borrow().iter().find(|t| t.page_num == page_num) {
            meta.widget.set_title(title);
        }
    }

    pub fn set_tab_favicon(&self, page_num: u32, icon_name: &str) {
        if let Some(meta) = self.tabs.borrow().iter().find(|t| t.page_num == page_num) {
            meta.widget.set_favicon_from_icon_name(icon_name);
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
