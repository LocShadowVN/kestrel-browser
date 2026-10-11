//! TabBar — tách tab strip (HBox) khỏi content (Notebook ẩn header) để
//! cho phép đặt toolbar giữa tab strip và content (giống Brave).

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Notebook, Orientation, Widget};

use crate::chrome::tab_widget::TabWidget;

struct TabMeta {
    widget: Rc<TabWidget>,
    page_num: u32,
}

pub struct TabBar {
    strip: GtkBox,
    notebook: Notebook,
    tabs: RefCell<Vec<Rc<TabMeta>>>,
    new_tab_button: Button,
}

impl TabBar {
    pub fn new() -> Self {
        let strip = GtkBox::new(Orientation::Horizontal, 0);
        strip.add_css_class("kestrel-tab-strip");
        strip.set_vexpand(false);
        strip.set_hexpand(true);

        let tabs_area = GtkBox::new(Orientation::Horizontal, 2);
        tabs_area.add_css_class("kestrel-tabs-area");
        tabs_area.set_hexpand(true);
        tabs_area.set_vexpand(false);

        let new_tab_button = Button::from_icon_name("tab-new-symbolic");
        new_tab_button.set_tooltip_text(Some("New tab"));
        new_tab_button.add_css_class("kestrel-new-tab-button");
        new_tab_button.set_vexpand(false);

        strip.append(&tabs_area);
        strip.append(&new_tab_button);

        let notebook = Notebook::new();
        notebook.set_vexpand(true);
        notebook.set_hexpand(true);
        notebook.set_show_tabs(false);
        notebook.set_show_border(false);
        notebook.add_css_class("kestrel-content");

        Self {
            strip,
            notebook,
            tabs: RefCell::new(Vec::new()),
            new_tab_button,
        }
    }

    pub fn add_tab<W, FC>(&self, content: &W, title: &str, on_close: FC) -> u32
    where
        W: IsA<Widget>,
        FC: Fn() + 'static,
    {
        let page_num_holder: Rc<std::cell::Cell<u32>> =
            Rc::new(std::cell::Cell::new(0));

        let tab = {
            let notebook_c = self.notebook.clone();
            let holder_c = page_num_holder.clone();
            TabWidget::new(
                title,
                move || {
                    let pn = holder_c.get();
                    notebook_c.set_current_page(Some(pn));
                },
                on_close,
            )
        };

        let page_num = self.notebook.append_page(content, None::<&gtk4::Widget>);
        page_num_holder.set(page_num);

        if let Some(tabs_area) = self.strip.first_child().and_downcast::<GtkBox>() {
            tabs_area.append(tab.widget());
        }

        self.tabs.borrow_mut().push(Rc::new(TabMeta {
            widget: tab,
            page_num,
        }));

        self.notebook.set_current_page(Some(page_num));
        self.update_active_classes();
        page_num
    }

    pub fn close_tab(&self, page_num: u32) {
        let removed_widget = self
            .tabs
            .borrow()
            .iter()
            .find(|t| t.page_num == page_num)
            .map(|t| t.widget.clone());

        if let Some(w) = &removed_widget {
            if let Some(parent) = w.widget().parent().and_downcast::<GtkBox>() {
                parent.remove(w.widget());
            }
        }

        if self.notebook.nth_page(Some(page_num)).is_some() {
            self.notebook.remove_page(Some(page_num));
        }
        self.tabs.borrow_mut().retain(|t| t.page_num != page_num);
        self.update_active_classes();
    }

    pub fn set_tab_title(&self, page_num: u32, title: &str) {
        if let Some(meta) = self.tabs.borrow().iter().find(|t| t.page_num == page_num) {
            meta.widget.set_title(title);
        }
    }

    pub fn set_tab_favicon(&self, page_num: u32, icon_name: &str) {
        if let Some(meta) = self.tabs.borrow().iter().find(|t| t.page_num == page_num) {
            meta.widget.set_favicon(icon_name);
        }
    }

    pub fn set_active(&self, index: usize) {
        self.notebook.set_current_page(Some(index as u32));
        self.update_active_classes();
    }

    pub fn active_index(&self) -> usize {
        self.notebook.current_page().unwrap_or(0) as usize
    }

    pub fn count(&self) -> usize {
        self.notebook.n_pages() as usize
    }

    fn update_active_classes(&self) {
        let current = self.notebook.current_page();
        for meta in self.tabs.borrow().iter() {
            meta.widget.set_active(Some(meta.page_num) == current);
        }
    }

    pub fn on_switch<F: Fn(u32) + 'static>(&self, cb: F) {
        let tabs_c = self.tabs.clone();
        self.notebook.connect_switch_page(move |_, _, page_num| {
            let current = Some(page_num);
            for meta in tabs_c.borrow().iter() {
                meta.widget.set_active(Some(meta.page_num) == current);
            }
            cb(page_num);
        });
    }

    pub fn on_new_tab<F: Fn() + 'static>(&self, cb: F) {
        self.new_tab_button.connect_clicked(move |_| cb());
    }

    pub fn strip_widget(&self) -> &GtkBox {
        &self.strip
    }

    pub fn content_widget(&self) -> &Notebook {
        &self.notebook
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self::new()
    }
}
