//! Tab widget — 2 button độc lập: thân tab (click để activate) và nút X
//! (click để close). Không dùng GestureClick để tránh xung đột event.

use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Image, Label, Orientation};

pub struct TabWidget {
    container: GtkBox,
    favicon: Image,
    label: Label,
    close_button: Button,
}

impl TabWidget {
    pub fn new<FA, FC>(title: &str, on_activate: FA, on_close: FC) -> Rc<Self>
    where
        FA: Fn() + 'static,
        FC: Fn() + 'static,
    {
        // Container ngoài — chỉ để gom body + close.
        let container = GtkBox::new(Orientation::Horizontal, 0);
        container.add_css_class("kestrel-tab");
        container.set_hexpand(false);
        container.set_vexpand(false);

        // --- Thân tab: Button chứa favicon + label ---
        let body = Button::new();
        body.add_css_class("kestrel-tab-body");
        body.set_hexpand(true);
        body.set_vexpand(false);

        let favicon = Image::from_icon_name("text-x-generic-symbolic");
        favicon.set_pixel_size(14);
        favicon.add_css_class("kestrel-tab-favicon");

        let label = Label::new(Some(title));
        label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        label.set_max_width_chars(20);
        label.set_single_line_mode(true);
        label.set_hexpand(true);
        label.set_xalign(0.0);
        label.add_css_class("kestrel-tab-label");

        let body_content = GtkBox::new(Orientation::Horizontal, 6);
        body_content.append(&favicon);
        body_content.append(&label);
        body.set_child(Some(&body_content));
        body.connect_clicked(move |_| on_activate());

        // --- Nút X: Button riêng, sibling của body ---
        let close_button = Button::from_icon_name("window-close-symbolic");
        close_button.set_tooltip_text(Some("Close tab"));
        close_button.add_css_class("kestrel-tab-close");
        close_button.set_vexpand(false);
        close_button.connect_clicked(move |_| on_close());

        container.append(&body);
        container.append(&close_button);

        Rc::new(Self {
            container,
            favicon,
            label,
            close_button,
        })
    }

    pub fn set_title(&self, title: &str) {
        let t = if title.is_empty() { "Untitled" } else { title };
        self.label.set_text(t);
    }

    pub fn set_favicon(&self, icon_name: &str) {
        self.favicon.set_icon_name(Some(icon_name));
    }

    pub fn set_active(&self, active: bool) {
        if active {
            self.container.add_css_class("active");
        } else {
            self.container.remove_css_class("active");
        }
    }

    pub fn widget(&self) -> &GtkBox {
        &self.container
    }

    #[allow(dead_code)]
    pub fn close_button(&self) -> &Button {
        &self.close_button
    }
}
