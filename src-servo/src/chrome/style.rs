//! Global CSS cho Kestrel — Tông màu trung tính, đầm mắt, hài hòa cả Dark Mode lẫn Light Mode.

use gtk4::prelude::*;

const CSS: &str = r#"
/* ============================================================
   WINDOW & THEME PALETTE
   ============================================================ */
window.kestrel-window {
    background-color: #111318;
    color: #e6e8f0;
}

window.kestrel-window.theme-light {
    background-color: #ebedf2;
    color: #181a20;
}

/* Chuyển các icon SVG đơn sắc sáng (#d4d6e0) sang màu than chì sắc nét trong Light Mode */
window.theme-light .kestrel-tab-strip image,
window.theme-light .kestrel-toolbar button.kestrel-toolbar-button image,
window.theme-light .kestrel-toolbar menubutton.kestrel-toolbar-button image,
window.theme-light .kestrel-omnibox-security,
window.theme-light .kestrel-omnibox-shield image,
window.theme-light popover image {
    filter: brightness(0.24);
}

window.theme-light button.kestrel-omnibox-action:not(.bookmarked) image {
    filter: brightness(0.28);
}

window.theme-light button.kestrel-omnibox-action.bookmarked image {
    filter: brightness(0.18);
}

/* ============================================================
   TAB STRIP
   ============================================================ */
.kestrel-tab-strip {
    background: #0e1015;
    padding: 5px 8px 4px 8px;
    min-height: 37px;
    border-bottom: 1px solid #1d202b;
}
window.theme-light .kestrel-tab-strip {
    background: #dfe2ea;
    border-bottom: 1px solid #ccd0dc;
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
    border-radius: 8px;
    min-height: 30px;
    min-width: 136px;
    max-width: 220px;
    margin: 0 2px;
    color: #8e93a6;
    transition: background-color 110ms ease, border-color 110ms ease, color 110ms ease;
}
.kestrel-tab:hover {
    background: rgba(255, 255, 255, 0.055);
    color: #d8dae4;
}
.kestrel-tab.active {
    background: #1b1e28;
    color: #f4f6fc;
    border-color: #2c3040;
}

window.theme-light .kestrel-tab {
    background: transparent;
    color: #525868;
}
window.theme-light .kestrel-tab:hover {
    background: rgba(0, 0, 0, 0.06);
    color: #1c1f26;
}
window.theme-light .kestrel-tab.active {
    background: #ffffff;
    color: #12141a;
    border-color: #c5cad6;
}

.kestrel-tab-body {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    outline: none;
    border-radius: 8px;
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
    border-radius: 6px;
    min-width: 20px;
    min-height: 20px;
    padding: 0;
    margin: 4px 5px 4px 2px;
    color: inherit;
    opacity: 0.45;
    transition: background-color 100ms ease, opacity 100ms ease;
}
.kestrel-tab:hover .kestrel-tab-close,
.kestrel-tab.active .kestrel-tab-close {
    opacity: 0.82;
}
.kestrel-tab-close:hover {
    background: rgba(255, 255, 255, 0.13);
    opacity: 1;
}
window.theme-light .kestrel-tab-close:hover {
    background: rgba(0, 0, 0, 0.09);
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
    transition: background-color 110ms ease;
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
    background: #171922;
    padding: 5px 10px;
    border-bottom: 1px solid #242836;
    min-height: 44px;
}
window.theme-light .kestrel-toolbar {
    background: #f5f6f9;
    border-bottom: 1px solid #d5d9e4;
}

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
    transition: background-color 100ms ease;
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
window.theme-light button.kestrel-toolbar-button:active,
window.theme-light menubutton.kestrel-toolbar-button > button:active,
window.theme-light menubutton.kestrel-toolbar-button > button:checked {
    background: rgba(0, 0, 0, 0.11);
}

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
   COMPOSITE OMNIBOX
   ============================================================ */
.kestrel-omnibox-box {
    background: #101218;
    border: 1px solid #282c3b;
    border-radius: 18px;
    padding: 1px 6px 1px 11px;
    min-height: 34px;
    margin: 0 8px;
    transition: border-color 120ms ease, background-color 120ms ease;
}
.kestrel-omnibox-box:focus-within {
    background: #12151d;
    border-color: #525975;
}
window.theme-light .kestrel-omnibox-box {
    background: #ffffff;
    border-color: #c9cfdc;
}
window.theme-light .kestrel-omnibox-box:focus-within {
    background: #ffffff;
    border-color: #6c738a;
}

.kestrel-omnibox-security {
    padding: 0 4px 0 0;
    opacity: 0.85;
}

entry.kestrel-omnibox-entry,
entry.kestrel-omnibox-entry:focus,
entry.kestrel-omnibox-entry > text {
    background: transparent;
    background-image: none;
    color: #eef0f7;
    border: none;
    box-shadow: none;
    outline: none;
    padding: 2px 4px;
    min-height: 24px;
    font-size: 13.5px;
    caret-color: #e4e6ef;
}
entry.kestrel-omnibox-entry > text > selection {
    background: #3d445c;
    color: #ffffff;
}
window.theme-light entry.kestrel-omnibox-entry,
window.theme-light entry.kestrel-omnibox-entry > text {
    color: #161820;
    caret-color: #161820;
}
window.theme-light entry.kestrel-omnibox-entry > text > selection {
    background: #cfe0ff;
    color: #111318;
}

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
    transition: background-color 100ms ease;
}
button.kestrel-omnibox-action:hover {
    background: rgba(255, 255, 255, 0.08);
}
window.theme-light button.kestrel-omnibox-action:hover {
    background: rgba(0, 0, 0, 0.07);
}

.kestrel-omnibox-divider {
    background: #272b3a;
    min-width: 1px;
    margin: 6px 3px;
}
window.theme-light .kestrel-omnibox-divider {
    background: #d6dae4;
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
    transition: background-color 100ms ease;
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
    color: #242834;
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
    background: #d0d3e0;
    border: none;
}
window.theme-light progressbar.kestrel-progress progress {
    background: #303442;
}

/* ============================================================
   CONTENT AREA & POPOVER MENU
   ============================================================ */
notebook.kestrel-content {
    background: #101217;
    border: none;
}
window.theme-light notebook.kestrel-content {
    background: #f4f5f8;
}

popover > contents {
    background: #181b24;
    border-radius: 12px;
    padding: 6px;
    border: 1px solid #2b2f3f;
    box-shadow: 0 10px 28px rgba(0, 0, 0, 0.45);
}
window.theme-light popover > contents {
    background: #ffffff;
    border: 1px solid #d2d6e2;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.14);
}

.kestrel-menu {
    background: transparent;
    padding: 4px;
    min-width: 224px;
}

.kestrel-shield-popover {
    min-width: 268px;
    padding: 6px;
}

.kestrel-shield-header {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid #292d3d;
    border-radius: 9px;
    padding: 10px 12px;
    margin-bottom: 8px;
}
window.theme-light .kestrel-shield-header {
    background: #f4f6fa;
    border: 1px solid #dfe3ec;
}

.kestrel-shield-stat-num {
    font-size: 16.5px;
    font-weight: 700;
    color: #f2f4fa;
}
window.theme-light .kestrel-shield-stat-num {
    color: #161820;
}

.kestrel-shield-stat-label {
    font-size: 11.5px;
    color: #8c90a4;
}
window.theme-light .kestrel-shield-stat-label {
    color: #5c6173;
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
    background: rgba(255, 255, 255, 0.075);
    color: #ffffff;
}
togglebutton.kestrel-menu-item:checked {
    background: rgba(255, 255, 255, 0.11);
    border-color: #3c4157;
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
window.theme-light togglebutton.kestrel-menu-item:checked {
    background: #e6e9f2;
    border-color: #b8bfd0;
    color: #111318;
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
