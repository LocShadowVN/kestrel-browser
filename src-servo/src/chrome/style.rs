//! Global CSS cho Kestrel — lấy cảm hứng từ Brave (dark theme mặc định).

use gtk4::prelude::*;

const CSS: &str = r#"
/* ============================================================
   WINDOW
   ============================================================ */
window.kestrel-window {
    background-color: #0d0d10;
}

/* ============================================================
   TAB STRIP — thanh tab trên cùng
   ============================================================ */
.kestrel-tab-strip {
    background: #0d0d10;
    padding: 6px 8px 0 8px;
    min-height: 38px;
    border-bottom: 1px solid #050506;
}
.kestrel-tabs-area {
    min-height: 32px;
}

/* Mỗi tab: container bo góc trên, active có nền sáng + viền dưới accent */
.kestrel-tab {
    background: transparent;
    border-radius: 10px 10px 0 0;
    min-height: 32px;
    min-width: 140px;
    max-width: 220px;
    margin: 0 2px;
    color: #8e8e96;
    transition: background 0.1s ease, color 0.1s ease;
}
.kestrel-tab:hover {
    background: #1f1f26;
    color: #c8c8d0;
}
.kestrel-tab.active {
    background: #26262c;
    color: #ffffff;
    box-shadow: inset 0 -2px 0 0 #4c8dff;
}

/* Thân tab: Button trong suốt */
.kestrel-tab-body {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 0;
    padding: 6px 4px 6px 12px;
    margin: 0;
    min-width: 0;
    min-height: 0;
    color: inherit;
    font-size: 12.5px;
}
.kestrel-tab-body:hover,
.kestrel-tab-body:active {
    background: transparent;
}

.kestrel-tab-favicon {
    color: inherit;
    opacity: 0.85;
}
.kestrel-tab.active .kestrel-tab-favicon {
    opacity: 1;
}

.kestrel-tab-label {
    font-size: 12.5px;
    color: inherit;
}

/* Nút X: chỉ hiện khi hover tab, giống Brave */
.kestrel-tab-close {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 5px;
    min-width: 20px;
    min-height: 20px;
    padding: 0;
    margin: 6px 6px 6px 0;
    color: inherit;
    opacity: 0;
    transition: opacity 0.1s ease, background 0.1s ease;
}
.kestrel-tab:hover .kestrel-tab-close,
.kestrel-tab.active .kestrel-tab-close {
    opacity: 0.55;
}
.kestrel-tab-close:hover {
    background: #3a3a44;
    opacity: 1;
    color: #ffffff;
}

/* Nút + thêm tab */
.kestrel-new-tab-button {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 8px;
    min-width: 30px;
    min-height: 30px;
    padding: 0;
    margin: 2px 4px 4px 4px;
    color: #a0a0a8;
    transition: background 0.1s ease, color 0.1s ease;
}
.kestrel-new-tab-button:hover {
    background: #26262c;
    color: #ffffff;
}

/* ============================================================
   CONTENT AREA
   ============================================================ */
notebook.kestrel-content {
    background: #ffffff;
    border: none;
}

/* ============================================================
   TOOLBAR — thanh địa chỉ
   ============================================================ */
.kestrel-toolbar {
    background: #1a1a1e;
    padding: 7px 12px;
    border-bottom: 1px solid #0a0a0c;
    min-height: 48px;
}

/* Nút toolbar: 34x34, bo góc 8px */
.kestrel-toolbar-button {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 8px;
    min-width: 34px;
    min-height: 34px;
    padding: 0;
    margin: 0 2px;
    color: #c8c8d0;
    transition: background 0.1s ease, color 0.1s ease;
}
.kestrel-toolbar-button:hover {
    background: #2a2a32;
    color: #ffffff;
}
.kestrel-toolbar-button:active,
.kestrel-toolbar-button:checked {
    background: #34343e;
    color: #ffffff;
}
.kestrel-toolbar-button:disabled {
    color: #5a5a64;
}

/* Ẩn arrow mặc định của MenuButton — chỉ giữ icon */
.kestrel-toolbar-button > arrow {
    -gtk-icon-source: none;
    min-width: 0;
    min-height: 0;
    padding: 0;
    margin: 0;
}

/* ============================================================
   OMNIBOX — pill shape
   ============================================================ */
.kestrel-omnibox {
    background: #26262c;
    color: #ffffff;
    border: 1px solid transparent;
    border-radius: 20px;
    padding: 7px 18px;
    min-height: 22px;
    font-size: 13.5px;
    caret-color: #4c8dff;
    margin: 0 8px;
    transition: background 0.15s ease, border-color 0.15s ease;
}
.kestrel-omnibox:focus,
.kestrel-omnibox:focus-within {
    background: #2f2f38;
    border-color: #4c8dff;
    box-shadow: 0 0 0 2px rgba(76, 141, 255, 0.15);
}
.kestrel-omnibox selection {
    background: #4c8dff;
    color: #ffffff;
}

/* ============================================================
   MENU POPOVER — hamburger + adblock popover
   ============================================================ */
.kestrel-menu {
    background: #1f1f26;
    border-radius: 12px;
    padding: 6px;
    border: 1px solid #2f2f38;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
}

.kestrel-menu-item {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 8px;
    padding: 9px 14px;
    color: #e0e0e6;
    font-size: 13px;
    transition: background 0.1s ease, color 0.1s ease;
}
.kestrel-menu-item:hover {
    background: #2f2f38;
    color: #ffffff;
}

.kestrel-menu-title {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 1.2px;
    color: #7a7a86;
    padding: 8px 14px 6px 14px;
}

.kestrel-menu-desc {
    font-size: 11px;
    color: #8a8a96;
    padding: 0 14px 10px 14px;
    line-height: 1.4;
}

.kestrel-menu-separator {
    background: #2f2f38;
    min-height: 1px;
    margin: 6px 10px;
}
"#;

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
