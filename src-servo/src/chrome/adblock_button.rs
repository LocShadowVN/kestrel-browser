//! Adblock button (Kestrel Shields) — Tích hợp trực tiếp bên trong thanh Omnibox như Brave.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Image, Label, MenuButton, Orientation, Popover, ToggleButton};

use crate::chrome::icons;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdblockLevel {
    Off,
    Standard,
    Aggressive,
}

impl AdblockLevel {
    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "Off (Shields Down)",
            Self::Standard => "Standard",
            Self::Aggressive => "Aggressive",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Off => "No network filtering",
            Self::Standard => "EasyList + EasyPrivacy",
            Self::Aggressive => "Standard + Fanboy's Annoyances",
        }
    }

    pub fn svg(self) -> &'static str {
        match self {
            Self::Off => icons::SHIELD_OFF,
            Self::Standard => icons::SHIELD_STANDARD,
            Self::Aggressive => icons::SHIELD_AGGRESSIVE,
        }
    }

    pub fn fallback_icon(self) -> &'static str {
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
    container: MenuButton,
    icon_img: Image,
    badge_label: Label,
    popover_stat_num: Label,
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
        let container = MenuButton::new();
        container.set_tooltip_text(Some("Kestrel Shields — Ad & Tracker Blocking"));
        container.add_css_class("kestrel-omnibox-shield");
        if initial == AdblockLevel::Off {
            container.add_css_class("shield-off");
        }

        let btn_content = GtkBox::new(Orientation::Horizontal, 4);
        let icon_img = icons::svg_image(initial.svg(), initial.fallback_icon(), 16);
        let badge_label = Label::new(Some("0"));
        badge_label.add_css_class("kestrel-shield-count");
        btn_content.append(&icon_img);
        btn_content.append(&badge_label);
        container.set_child(Some(&btn_content));

        let current = Rc::new(Cell::new(initial));
        let on_change: Rc<dyn Fn(u8)> = Rc::new(on_change);

        let popover = Popover::new();
        popover.set_has_arrow(false);

        let menu = GtkBox::new(Orientation::Vertical, 0);
        menu.add_css_class("kestrel-menu");
        menu.add_css_class("kestrel-shield-popover");

        let header = GtkBox::new(Orientation::Vertical, 2);
        header.add_css_class("kestrel-shield-header");
        let popover_stat_num = Label::new(Some("0"));
        popover_stat_num.add_css_class("kestrel-shield-stat-num");
        popover_stat_num.set_xalign(0.0);
        let stat_desc = Label::new(Some("Ads & trackers blocked on this page"));
        stat_desc.add_css_class("kestrel-shield-stat-label");
        stat_desc.set_xalign(0.0);
        header.append(&popover_stat_num);
        header.append(&stat_desc);
        menu.append(&header);

        let title = Label::new(Some("Shields Protection Level"));
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
            let icon_img_c = icon_img.clone();
            btn.connect_toggled(move |b| {
                if b.is_active() && current_c.get() != level {
                    current_c.set(level);
                    icons::set_image_svg(&icon_img_c, level.svg(), level.fallback_icon(), 16);
                    if level == AdblockLevel::Off {
                        container_c.add_css_class("shield-off");
                    } else {
                        container_c.remove_css_class("shield-off");
                    }
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
            icon_img,
            badge_label,
            popover_stat_num,
            popover,
            current,
            toggles,
        }
    }

    pub fn widget(&self) -> &MenuButton {
        &self.container
    }

    pub fn set_level(&self, level: u8) {
        let l = AdblockLevel::from_u8(level);
        self.current.set(l);
        icons::set_image_svg(&self.icon_img, l.svg(), l.fallback_icon(), 16);
        if l == AdblockLevel::Off {
            self.container.add_css_class("shield-off");
        } else {
            self.container.remove_css_class("shield-off");
        }
        for (lv, btn) in self.toggles.borrow().iter() {
            if btn.is_active() != (*lv == l) {
                btn.set_active(*lv == l);
            }
        }
    }

    pub fn set_blocked_count(&self, tab_blocked: u32, total_blocked: u64) {
        self.badge_label.set_text(&tab_blocked.to_string());
        self.popover_stat_num
            .set_text(&format!("{} ({} session total)", tab_blocked, total_blocked));
    }

    #[allow(dead_code)]
    pub fn current_level(&self) -> AdblockLevel {
        self.current.get()
    }
}
