//! Global CSS cho Kestrel — Tông màu trung tính, đơn sắc, tối giản kiểu Brave.
//! Xử lý triệt để các node con `menubutton > button` và `entry > text` của GTK4.

use gtk4::prelude::*;

const CSS: &str = r#"
/* ============================================================
   WINDOW & THEME PALETTE
   ============================================================ */
window.kestrel-window {
    background-color: #0e0f14;
    color: #e4e6ef;
}

window.kestrel-window.theme-light {
    background-color: #e9ebf0;
    color: #181a20;
}

/* ============================================================
   TAB STRIP
   ============================================================ */
.kestrel-tab-strip {
    background: #0e0f14;
    padding: 5px 8px 3px 8px;
    min-height: 36px;
    border-bottom: 1px solid #1a1c24;
}
window.theme-light .kestrel-tab-strip {
    background: #dfe2e8;
    border-bottom: 1px solid #cfd3dc;
}

.kestrel-tabs-scroll {
    background: transparent;
    border: none;
}

.kestrel-tabs-area {
    min-height: 30px;
}

.kestrel-tab {
    background: transparent;
    border: 1px solid transparent;
    border-radius: 7px;
    min-height: 30px;
    min-width: 130px;
    max-width: 210px;
    margin: 0 2px;
    color: #8c90a4;
}
.kestrel-tab:hover {
    background: rgba(255, 255, 255, 0.05);
    color: #d4d6e0;
}
.kestrel-tab.active {
    background: #1b1d26;
    color: #f2f4fa;
    border-color: #2a2d3a;
}

window.theme-light .kestrel-tab {
    background: transparent;
    color: #525866;
}
window.theme-light .kestrel-tab:hover {
    background: rgba(0, 0, 0, 0.06);
    color: #1e2028;
}
window.theme-light .kestrel-tab.active {
    background: #ffffff;
    color: #111318;
    border-color: #c9ced8;
}

.kestrel-tab-body {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    outline: none;
    border-radius: 7px;
    padding: 3px 4px 3px 10px;
    margin: 0;
    min-width: 0;
    min-height: 0;
    color: inherit;
    font-size: 12.5px;
    font-weight: 500;
}
.kestrel-tab-body:hover,
.kestrel-tab-body:active {
    background: transparent;
    box-shadow: none;
}

.kestrel-tab-favicon {
    opacity: 0.78;
}
.kestrel-tab.active .kestrel-tab-favicon {
    opacity: 1;
}

.kestrel-tab-label {
    font-size: 12.5px;
    color: inherit;
}

.kestrel-tab-close {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    outline: none;
    border-radius: 5px;
    min-width: 20px;
    min-height: 20px;
    padding: 0;
    margin: 4px 5px 4px 2px;
    color: inherit;
    opacity: 0.4;
}
.kestrel-tab:hover .kestrel-tab-close,
.kestrel-tab.active .kestrel-tab-close {
    opacity: 0.8;
}
.kestrel-tab-close:hover {
    background: rgba(255, 255, 255, 0.12);
    opacity: 1;
}

.kestrel-new-tab-button {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    outline: none;
    border-radius: 7px;
    min-width: 30px;
    min-height: 30px;
    padding: 0;
    margin: 0 2px 0 4px;
    color: #c8cad4;
}
.kestrel-new-tab-button:hover {
    background: rgba(255, 255, 255, 0.08);
}
window.theme-light .kestrel-new-tab-button:hover {
    background: rgba(0, 0, 0, 0.07);
}

/* ============================================================
   TOOLBAR
   ============================================================ */
.kestrel-toolbar {
    background: #161820;
    padding: 5px 10px;
    border-bottom: 1px solid #222532;
    min-height: 44px;
}
window.theme-light .kestrel-toolbar {
    background: #f5f6f8;
    border-bottom: 1px solid #d8dce6;
}

/* Reset cả Button thường lẫn nút con bên trong MenuButton của GTK4 */
button.kestrel-toolbar-button,
menubutton.kestrel-toolbar-button,
menubutton.kestrel-toolbar-button > button {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    outline: none;
    border-radius: 7px;
    min-width: 32px;
    min-height: 32px;
    padding: 0 5px;
    margin: 0 1px;
    color: #d4d6e0;
}
button.kestrel-toolbar-button:hover,
menubutton.kestrel-toolbar-button > button:hover {
    background: rgba(255, 255, 255, 0.08);
}
button.kestrel-toolbar-button:active,
button.kestrel-toolbar-button:checked,
menubutton.kestrel-toolbar-button > button:active,
menubutton.kestrel-toolbar-button > button:checked {
    background: rgba(255, 255, 255, 0.13);
}
window.theme-light button.kestrel-toolbar-button:hover,
window.theme-light menubutton.kestrel-toolbar-button > button:hover {
    background: rgba(0, 0, 0, 0.07);
}

/* Ẩn arrow mặc định của MenuButton */
menubutton.kestrel-toolbar-button > button > arrow,
menubutton.kestrel-omnibox-shield > button > arrow,
.kestrel-toolbar-button > arrow,
.kestrel-omnibox-shield > arrow {
    -gtk-icon-source: none;
    min-width: 0;
    min-height: 0;
    padding: 0;
    margin: 0;
}

/* ============================================================
   COMPOSITE OMNIBOX (Tích hợp Lock + Entry + Star + Shields)
   ============================================================ */
.kestrel-omnibox-box {
    background: #0f1016;
    border: 1px solid #272a38;
    border-radius: 18px;
    padding: 1px 6px 1px 11px;
    min-height: 34px;
    margin: 0 8px;
}
.kestrel-omnibox-box:focus-within {
    background: #11131a;
    border-color: #4a5068;
}
window.theme-light .kestrel-omnibox-box {
    background: #ffffff;
    border-color: #cdd2de;
}
window.theme-light .kestrel-omnibox-box:focus-within {
    background: #ffffff;
    border-color: #7a8094;
}

.kestrel-omnibox-security {
    padding: 0 4px 0 0;
    opacity: 0.82;
}

/* Xóa hoàn toàn khung nền mặc định của GTK4 Entry và node text bên trong */
entry.kestrel-omnibox-entry,
entry.kestrel-omnibox-entry:focus,
entry.kestrel-omnibox-entry > text {
    background: transparent;
    background-image: none;
    color: #eceef5;
    border: none;
    box-shadow: none;
    outline: none;
    padding: 2px 4px;
    min-height: 24px;
    font-size: 13.5px;
    caret-color: #e4e6ef;
}
entry.kestrel-omnibox-entry > text > selection {
    background: #3b4054;
    color: #ffffff;
}
window.theme-light entry.kestrel-omnibox-entry,
window.theme-light entry.kestrel-omnibox-entry > text {
    color: #151820;
    caret-color: #151820;
}

/* Nút Star và nút Shields nằm gọn bên phải trong lòng Omnibox */
button.kestrel-omnibox-action {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    outline: none;
    border-radius: 12px;
    min-width: 26px;
    min-height: 26px;
    padding: 0;
    margin: 0 1px;
}
button.kestrel-omnibox-action:hover {
    background: rgba(255, 255, 255, 0.08);
}
window.theme-light button.kestrel-omnibox-action:hover {
    background: rgba(0, 0, 0, 0.06);
}

.kestrel-omnibox-divider {
    background: #272a38;
    min-width: 1px;
    margin: 6px 3px;
}
window.theme-light .kestrel-omnibox-divider {
    background: #d8dce6;
}

menubutton.kestrel-omnibox-shield,
menubutton.kestrel-omnibox-shield > button {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    outline: none;
    border-radius: 13px;
    min-height: 26px;
    padding: 0 7px;
    margin: 0;
}
menubutton.kestrel-omnibox-shield > button:hover,
menubutton.kestrel-omnibox-shield > button:checked {
    background: rgba(255, 255, 255, 0.09);
}
window.theme-light menubutton.kestrel-omnibox-shield > button:hover,
window.theme-light menubutton.kestrel-omnibox-shield > button:checked {
    background: rgba(0, 0, 0, 0.07);
}

.kestrel-omnibox-shield.shield-off {
    opacity: 0.55;
}

.kestrel-shield-count {
    font-size: 11.5px;
    font-weight: 600;
    color: #d4d6e0;
}
window.theme-light .kestrel-shield-count {
    color: #222530;
}

/* ============================================================
   LOADING PROGRESS BAR
   ============================================================ */
progressbar.kestrel-progress {
    min-height: 2px;
    padding: 0;
    margin: 0;
    background: transparent;
}
progressbar.kestrel-progress trough {
    min-height: 2px;
    background: transparent;
    border: none;
}
progressbar.kestrel-progress progress {
    min-height: 2px;
    background: #c8cad4;
    border: none;
}

/* ============================================================
   CONTENT AREA & POPOVER MENU
   ============================================================ */
notebook.kestrel-content {
    background: #101116;
    border: none;
}

popover > contents {
    background: #181a22;
    border-radius: 12px;
    padding: 6px;
    border: 1px solid #2a2d3c;
    box-shadow: 0 10px 28px rgba(0, 0, 0, 0.45);
}
window.theme-light popover > contents {
    background: #ffffff;
    border: 1px solid #d4d8e2;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.14);
}

.kestrel-menu {
    background: transparent;
    padding: 4px;
    min-width: 220px;
}

.kestrel-shield-popover {
    min-width: 265px;
    padding: 6px;
}

.kestrel-shield-header {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid #282b3a;
    border-radius: 8px;
    padding: 10px 12px;
    margin-bottom: 8px;
}

.kestrel-shield-stat-num {
    font-size: 17px;
    font-weight: 700;
    color: #f2f4fa;
}
window.theme-light .kestrel-shield-stat-num {
    color: #16181f;
}

.kestrel-shield-stat-label {
    font-size: 11.5px;
    color: #8c90a4;
}

button.kestrel-menu-item,
togglebutton.kestrel-menu-item {
    background: transparent;
    background-image: none;
    border: 1px solid transparent;
    box-shadow: none;
    outline: none;
    border-radius: 7px;
    padding: 7px 11px;
    color: #dfe1eb;
    font-size: 13px;
    font-weight: 500;
}
button.kestrel-menu-item:hover,
togglebutton.kestrel-menu-item:hover {
    background: rgba(255, 255, 255, 0.07);
    color: #ffffff;
}
togglebutton.kestrel-menu-item:checked {
    background: rgba(255, 255, 255, 0.11);
    border-color: #3a3e52;
    color: #ffffff;
}
window.theme-light button.kestrel-menu-item,
window.theme-light togglebutton.kestrel-menu-item {
    color: #1d202a;
}
window.theme-light button.kestrel-menu-item:hover,
window.theme-light togglebutton.kestrel-menu-item:hover {
    background: #eef0f6;
    color: #000000;
}

.kestrel-menu-shortcut {
    font-size: 11px;
    color: #7c8196;
    font-family: monospace;
}

.kestrel-menu-title {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    color: #858a9e;
    padding: 6px 11px 4px 11px;
}

.kestrel-menu-desc {
    font-size: 11px;
    color: #8c91a6;
    padding: 0 11px 8px 11px;
}

.kestrel-menu-separator {
    background: #282b3a;
    min-height: 1px;
    margin: 5px 8px;
}
window.theme-light .kestrel-menu-separator {
    background: #e1e4ec;
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
