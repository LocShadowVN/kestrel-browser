//! Adblock button — shield icon + popover chọn mức độ chặn.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation, Popover, ToggleButton};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdblockLevel {
    Off,
    Standard,
    Aggressive,
}

impl AdblockLevel {
    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Standard => "Standard",
            Self::Aggressive => "Aggressive",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Off => "No filtering",
            Self::Standard => "EasyList + EasyPrivacy",
            Self::Aggressive => "Standard + Fanboy's Annoyances",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Off => "security-low-symbolic",
            Self::Standard => "security-medium-symbolic",
            Self::Aggressive => "security-high-symbolic",
        }
    }

    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Standard,
            2 => Self::Aggressive,
            _ => Self::Off,
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            Self::Off => 0,
            Self::Standard => 1,
            Self::Aggressive => 2,
        }
    }
}

pub struct AdblockButton {
    container: Button,
    #[allow(dead_code)]
    popover: Popover,
    current: Rc<Cell<AdblockLevel>>,
    toggles: Rc<RefCell<Vec<(AdblockLevel, ToggleButton)>>>,
}

impl AdblockButton {
    pub fn new<F>(initial: AdblockLevel, on_change: F) -> Self
    where
        F: Fn(u8) + 'static,
    {
        let container = Button::from_icon_name(initial.icon());
        container.set_tooltip_text(Some("Ad blocking"));
        container.add_css_class("kestrel-toolbar-button");

        let current = Rc::new(Cell::new(initial));
        let on_change: Rc<dyn Fn(u8)> = Rc::new(on_change);

        let popover = Popover::new();
        popover.set_has_arrow(false);

        let menu = GtkBox::new(Orientation::Vertical, 0);
        menu.add_css_class("kestrel-menu");

        let title = Label::new(Some("Ad blocking level"));
        title.add_css_class("kestrel-menu-title");
        title.set_xalign(0.0);
        menu.append(&title);

        let toggles: Rc<RefCell<Vec<(AdblockLevel, ToggleButton)>>> =
            Rc::new(RefCell::new(Vec::new()));
        let mut first_toggle: Option<ToggleButton> = None;

        for level in [
            AdblockLevel::Off,
            AdblockLevel::Standard,
            AdblockLevel::Aggressive,
        ] {
            let btn = ToggleButton::with_label(level.label());
            btn.add_css_class("kestrel-menu-item");
            btn.set_active(level == initial);

            if let Some(first) = &first_toggle {
                btn.set_group(Some(first));
            } else {
                first_toggle = Some(btn.clone());
            }

            let current_c = current.clone();
            let on_change_c = on_change.clone();
            let popover_c = popover.clone();
            let container_c = container.clone();
            btn.connect_toggled(move |b| {
                if b.is_active() {
                    current_c.set(level);
                    container_c.set_icon_name(level.icon());
                    (on_change_c)(level.as_u8());
                    popover_c.popdown();
                }
            });

            menu.append(&btn);

            let desc = Label::new(Some(level.description()));
            desc.add_css_class("kestrel-menu-desc");
            desc.set_xalign(0.0);
            menu.append(&desc);

            toggles.borrow_mut().push((level, btn));
        }

        popover.set_child(Some(&menu));
        container.set_popover(Some(&popover));

        Self {
            container,
            popover,
            current,
            toggles,
        }
    }

    pub fn widget(&self) -> &Button {
        &self.container
    }

    /// Cập nhật level mà không gọi callback.
    pub fn set_level(&self, level: u8) {
        let l = AdblockLevel::from_u8(level);
        self.current.set(l);
        self.container.set_icon_name(l.icon());
        for (lv, btn) in self.toggles.borrow().iter() {
            btn.set_active(*lv == l);
        }
    }

    #[allow(dead_code)]
    pub fn current_level(&self) -> AdblockLevel {
        self.current.get()
    }
}
