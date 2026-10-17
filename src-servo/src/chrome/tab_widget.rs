//! Tab widget — thân tab (click để activate), nút X SVG (click để close),
//! và hỗ trợ click chuột giữa (Middle-click) để đóng tab như trình duyệt thật.

use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, EventSequenceState, GestureClick, Image, Label, Orientation};

use crate::chrome::icons;

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
        let on_close = Rc::new(on_close);

        let container = GtkBox::new(Orientation::Horizontal, 0);
        container.add_css_class("kestrel-tab");
        container.set_hexpand(false);
        container.set_vexpand(false);

        // --- Thân tab: Button chứa SVG favicon + label ---
        let body = Button::new();
        body.add_css_class("kestrel-tab-body");
        body.set_hexpand(true);
        body.set_vexpand(false);

        let favicon = icons::svg_image(icons::GLOBE, "web-browser-symbolic", 15);
        favicon.add_css_class("kestrel-tab-favicon");

        let label = Label::new(Some(title));
        label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        label.set_max_width_chars(20);
        label.set_single_line_mode(true);
        label.set_hexpand(true);
        label.set_xalign(0.0);
        label.add_css_class("kestrel-tab-label");

        let body_content = GtkBox::new(Orientation::Horizontal, 7);
        body_content.append(&favicon);
        body_content.append(&label);
        body.set_child(Some(&body_content));
        body.connect_clicked(move |_| on_activate());

        // --- Nút X đóng tab bằng SVG ---
        let close_button = icons::svg_button(
            icons::CLOSE,
            "window-close-symbolic",
            13,
            "Close tab (Ctrl+W)",
        );
        close_button.add_css_class("kestrel-tab-close");
        close_button.set_vexpand(false);
        {
            let on_close_c = on_close.clone();
            close_button.connect_clicked(move |_| on_close_c());
        }

        // --- Hỗ trợ Middle-click (nút chuột 2) để đóng tab ---
        let middle_click = GestureClick::new();
        middle_click.set_button(2);
        {
            let on_close_c = on_close.clone();
            middle_click.connect_released(move |gesture, _n_press, _x, _y| {
                gesture.set_state(EventSequenceState::Claimed);
                on_close_c();
            });
        }
        container.add_controller(middle_click);

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
        let t = if title.trim().is_empty() {
            "New Tab"
        } else {
            title
        };
        self.label.set_text(t);
        self.container.set_tooltip_text(Some(t));
    }

    pub fn set_favicon_svg(&self, svg: &'static str, fallback_icon: &str) {
        icons::set_image_svg(&self.favicon, svg, fallback_icon, 15);
    }

    pub fn set_favicon(&self, icon_name: &str) {
        if icon_name == "web-browser-symbolic" {
            self.set_favicon_svg(icons::KESTREL_BADGE, icon_name);
        } else {
            self.set_favicon_svg(icons::GLOBE, icon_name);
        }
    }

    pub fn set_loading(&self, loading: bool, is_internal: bool) {
        if loading {
            self.set_favicon_svg(icons::SPINNER_LOADING, "content-loading-symbolic");
        } else if is_internal {
            self.set_favicon_svg(icons::KESTREL_BADGE, "web-browser-symbolic");
        } else {
            self.set_favicon_svg(icons::GLOBE, "text-x-generic-symbolic");
        }
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
