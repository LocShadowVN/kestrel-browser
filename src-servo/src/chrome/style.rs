//! Global CSS cho Kestrel — phong cách Brave-like.
//!
//! Được load một lần khi app khởi động, áp dụng cho mọi widget có CSS class
//! tương ứng. Không dùng CSS của GTK theme mặc định để đảm bảo giao diện
//! nhất quán trên mọi distro.

use gtk4::prelude::*;

const CSS: &str = r#"
/* ---------- Toolbar ---------- */
.kestrel-toolbar {
    background-color: #1c1c22;
    padding: 6px 8px;
    border-bottom: 1px solid #0d0d12;
}

.kestrel-toolbar-button {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 8px;
    min-width: 32px;
    min-height: 32px;
    padding: 0;
    margin: 0 2px;
    color: #cfcfd6;
}
.kestrel-toolbar-button:hover {
    background: #2f2f38;
    color: #ffffff;
}
.kestrel-toolbar-button:active,
.kestrel-toolbar-button:checked {
    background: #3a3a45;
    color: #ffffff;
}

/* ---------- Omnibox ---------- */
.kestrel-omnibox {
    background: #2a2a35;
    color: #ffffff;
    border: 1px solid transparent;
    border-radius: 18px;
    padding: 6px 14px;
    min-height: 22px;
    font-size: 13px;
    caret-color: #ffffff;
}
.kestrel-omnibox:focus,
.kestrel-omnibox:focus-within {
    background: #313140;
    border-color: #5b8dee;
}
.kestrel-omnibox selection {
    background: #3a5bcf;
    color: #ffffff;
}

/* ---------- Tab bar ---------- */
notebook.kestrel-tabbar > header {
    background: #14141a;
    border-bottom: none;
    padding: 4px 4px 0 4px;
}
notebook.kestrel-tabbar > header > tabs > tab {
    background: transparent;
    border: none;
    border-radius: 6px 6px 0 0;
    padding: 6px 12px;
    color: #8e8e93;
    min-height: 26px;
    margin: 0 1px;
}
notebook.kestrel-tabbar > header > tabs > tab:checked {
    background: #1c1c22;
    color: #ffffff;
}
notebook.kestrel-tabbar > header > tabs > tab:hover {
    background: #1e1e25;
}

/* ---------- Menu popover ---------- */
.kestrel-menu {
    background: #1f1f28;
    border-radius: 10px;
    padding: 6px;
    border: 1px solid #2f2f3a;
}

.kestrel-menu-item {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 6px;
    padding: 8px 14px;
    color: #e0e0e6;
    font-size: 13px;
}
.kestrel-menu-item:hover {
    background: #2f2f3a;
    color: #ffffff;
}

.kestrel-menu-separator {
    background: #2f2f3a;
    min-height: 1px;
    margin: 4px 8px;
}

/* ---------- Window ---------- */
window.kestrel-window {
    background-color: #14141a;
}
"#;

/// Load CSS vào display mặc định. Gọi một lần khi app khởi động.
pub fn load() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(CSS);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
