//! Adblock button — shield icon + popover chọn mức độ chặn.
//!
//! Hiện tại UI-only: callback lưu level vào Config. Khi fork servo-gtk
//! hoàn tất, callback này cũng gọi webview.set_adblock_level().

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
            Self::Standard => "EasyList — blocks ads",
            Self::Aggressive => "EasyList + EasyPrivacy + uBO filters",
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
}

impl AdblockButton {
    pub fn new<F>(initial: AdblockLevel, on_change: F) -> Self
    where
        F: Fn(AdblockLevel) + 'static,
    {
        let container = Button::from_icon_name(initial.icon());
        container.set_tooltip_text(Some("Ad blocking"));
        container.add_css_class("kestrel-toolbar-button");

        let current = Rc::new(Cell::new(initial));
        let on_change: Rc<dyn Fn(AdblockLevel)> = Rc::new(on_change);

        let popover = Popover::new();
        popover.set_has_arrow(false);

        let menu = GtkBox::new(Orientation::Vertical, 0);
        menu.add_css_class("kestrel-menu");

        let title = Label::new(Some("Ad blocking level"));
        title.add_css_class("kestrel-menu-title");
        title.set_xalign(0.0);
        menu.append(&title);

        let toggles: Rc<RefCell<Vec<ToggleButton>>> = Rc::new(RefCell::new(Vec::new()));

        for level in [AdblockLevel::Off, AdblockLevel::Standard, AdblockLevel::Aggressive] {
            let btn = ToggleButton::with_label(level.label());
            btn.add_css_class("kestrel-menu-item");
            btn.set_active(level == initial);

            // Group: chỉ 1 active tại một thời điểm.
            {
                let borrowed = toggles.borrow();
                if let Some(first) = borrowed.first() {
                    btn.set_group(Some(first));
                }
            }
            toggles.borrow_mut().push(btn.clone());

            let current_c = current.clone();
            let on_change_c = on_change.clone();
            let popover_c = popover.clone();
            let container_c = container.clone();
            let level_c = level;
            btn.connect_toggled(move |b| {
                if b.is_active() {
                    current_c.set(level_c);
                    container_c.set_icon_name(level_c.icon());
                    (on_change_c)(level_c);
                    popover_c.popdown();
                }
            });

            menu.append(&btn);

            let desc = Label::new(Some(level.description()));
            desc.add_css_class("kestrel-menu-desc");
            desc.set_xalign(0.0);
            menu.append(&desc);
        }

        popover.set_child(Some(&menu));
        container.set_popover(Some(&popover));

        Self { container, popover, current }
    }

    pub fn widget(&self) -> &Button {
        &self.container
    }

    pub fn set_level(&self, level: AdblockLevel) {
        self.current.set(level);
        self.container.set_icon_name(level.icon());
    }

    #[allow(dead_code)]
    pub fn current_level(&self) -> AdblockLevel {
        self.current.get()
    }
}
