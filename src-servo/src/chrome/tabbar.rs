//! TabBar — Quản lý các tab bằng `tab_id: u64` duy nhất và tra cứu động `notebook.page_num`.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Notebook, Orientation, PolicyType, ScrolledWindow, Widget};

use crate::chrome::icons;
use crate::chrome::tab_widget::TabWidget;

struct TabMeta {
    tab_id: u64,
    widget: Rc<TabWidget>,
    content: Widget,
}

pub struct TabBar {
    strip: GtkBox,
    tabs_area: GtkBox,
    notebook: Notebook,
    tabs: Rc<RefCell<Vec<Rc<TabMeta>>>>,
    next_id: Cell<u64>,
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

        let scroll = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Automatic)
            .vscrollbar_policy(PolicyType::Never)
            .hexpand(true)
            .vexpand(false)
            .build();
        scroll.add_css_class("kestrel-tabs-scroll");
        scroll.set_child(Some(&tabs_area));

        let new_tab_button =
            icons::svg_button(icons::PLUS, "tab-new-symbolic", 16, "New Tab (Ctrl+T)");
        new_tab_button.add_css_class("kestrel-new-tab-button");
        new_tab_button.set_vexpand(false);

        strip.append(&scroll);
        strip.append(&new_tab_button);

        let notebook = Notebook::new();
        notebook.set_vexpand(true);
        notebook.set_hexpand(true);
        notebook.set_show_tabs(false);
        notebook.set_show_border(false);
        notebook.add_css_class("kestrel-content");

        Self {
            strip,
            tabs_area,
            notebook,
            tabs: Rc::new(RefCell::new(Vec::new())),
            next_id: Cell::new(1),
            new_tab_button,
        }
    }

    pub fn add_tab<W, FC>(&self, content: &W, title: &str, on_close: FC) -> u64
    where
        W: IsA<Widget>,
        FC: Fn(u64) + 'static,
    {
        let tab_id = self.next_id.get();
        self.next_id.set(tab_id + 1);

        let content_widget: Widget = content.clone().upcast();

        let tab = {
            let notebook_c = self.notebook.clone();
            let content_c = content_widget.clone();
            TabWidget::new(
                title,
                move || {
                    if let Some(pn) = notebook_c.page_num(&content_c) {
                        notebook_c.set_current_page(Some(pn));
                    }
                },
                move || {
                    on_close(tab_id);
                },
            )
        };

        self.tabs_area.append(tab.widget());

        // Đẩy vào danh sách TRƯỚC khi gọi append_page vì append_page bắn sự kiện switch-page ngay lập tức
        self.tabs.borrow_mut().push(Rc::new(TabMeta {
            tab_id,
            widget: tab,
            content: content_widget.clone(),
        }));

        let page_num = self.notebook.append_page(&content_widget, None::<&Widget>);
        self.notebook.set_current_page(Some(page_num));
        self.update_active_classes();
        tab_id
    }

    pub fn close_tab(&self, tab_id: u64) {
        let meta = self
            .tabs
            .borrow()
            .iter()
            .find(|t| t.tab_id == tab_id)
            .cloned();

        if let Some(m) = meta {
            self.tabs_area.remove(m.widget.widget());
            if let Some(pn) = self.notebook.page_num(&m.content) {
                self.notebook.remove_page(Some(pn));
            }
            self.tabs.borrow_mut().retain(|t| t.tab_id != tab_id);
            self.update_active_classes();
        }
    }

    pub fn set_tab_title(&self, tab_id: u64, title: &str) {
        if let Some(meta) = self.tabs.borrow().iter().find(|t| t.tab_id == tab_id) {
            meta.widget.set_title(title);
        }
    }

    pub fn set_tab_favicon(&self, tab_id: u64, icon_name: &str) {
        if let Some(meta) = self.tabs.borrow().iter().find(|t| t.tab_id == tab_id) {
            meta.widget.set_favicon(icon_name);
        }
    }

    pub fn set_tab_loading(&self, tab_id: u64, loading: bool, is_internal: bool) {
        if let Some(meta) = self.tabs.borrow().iter().find(|t| t.tab_id == tab_id) {
            meta.widget.set_loading(loading, is_internal);
        }
    }

    pub fn set_active_by_id(&self, tab_id: u64) {
        let page_num = self
            .tabs
            .borrow()
            .iter()
            .find(|t| t.tab_id == tab_id)
            .and_then(|m| self.notebook.page_num(&m.content));

        if let Some(pn) = page_num {
            self.notebook.set_current_page(Some(pn));
            self.update_active_classes();
        }
    }

    pub fn set_active(&self, index: usize) {
        if index < self.count() {
            self.notebook.set_current_page(Some(index as u32));
            self.update_active_classes();
        }
    }

    pub fn active_tab_id(&self) -> Option<u64> {
        let current = self.notebook.current_page()?;
        let page_widget = self.notebook.nth_page(Some(current))?;
        self.tabs
            .borrow()
            .iter()
            .find(|m| m.content == page_widget)
            .map(|m| m.tab_id)
    }

    pub fn active_index(&self) -> usize {
        self.notebook.current_page().unwrap_or(0) as usize
    }

    pub fn count(&self) -> usize {
        self.notebook.n_pages() as usize
    }

    fn update_active_classes(&self) {
        let current_widget = self
            .notebook
            .current_page()
            .and_then(|pn| self.notebook.nth_page(Some(pn)));

        for meta in self.tabs.borrow().iter() {
            let is_active = current_widget
                .as_ref()
                .map(|w| *w == meta.content)
                .unwrap_or(false);
            meta.widget.set_active(is_active);
        }
    }

    pub fn on_switch<F: Fn(u64) + 'static>(&self, cb: F) {
        let tabs_c = self.tabs.clone();
        self.notebook
            .connect_switch_page(move |_notebook, page_widget, _page_num| {
                let mut switched_id = None;
                for meta in tabs_c.borrow().iter() {
                    let is_active = meta.content == *page_widget;
                    meta.widget.set_active(is_active);
                    if is_active {
                        switched_id = Some(meta.tab_id);
                    }
                }
                if let Some(id) = switched_id {
                    cb(id);
                }
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
