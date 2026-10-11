//! Custom tab widget — giống Brave: favicon + title + nút X.

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Image, Label, Orientation};

pub struct TabWidget {
    container: GtkBox,
    favicon: Image,
    label: Label,
    close_button: Button,
}

impl TabWidget {
    pub fn new(title: &str, on_close: impl Fn() + 'static) -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 6);
        container.add_css_class("kestrel-tab-content");
        container.set_hexpand(true);

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

        let close_button = Button::from_icon_name("window-close-symbolic");
        close_button.set_tooltip_text(Some("Close tab"));
        close_button.add_css_class("kestrel-tab-close");
        close_button.connect_clicked(move |_| on_close());

        container.append(&favicon);
        container.append(&label);
        container.append(&close_button);

        Self {
            container,
            favicon,
            label,
            close_button,
        }
    }

    pub fn set_title(&self, title: &str) {
        let t = if title.is_empty() { "Untitled" } else { title };
        self.label.set_text(t);
    }

    pub fn set_favicon_from_icon_name(&self, icon_name: &str) {
        self.favicon.set_icon_name(Some(icon_name));
    }

    pub fn widget(&self) -> &GtkBox {
        &self.container
    }

    #[allow(dead_code)]
    pub fn favicon(&self) -> &Image {
        &self.favicon
    }

    #[allow(dead_code)]
    pub fn close_button(&self) -> &Button {
        &self.close_button
    }
}
