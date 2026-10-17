//! Global CSS cho Kestrel — Tông màu trung tính, đơn sắc, tối giản kiểu Brave.

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
    padding: 6px 8px 4px 8px;
    min-height: 38px;
    border-bottom: 1px solid #1b1d26;
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
    min-height: 32px;
}

.kestrel-tab {
    background: transparent;
    border: 1px solid transparent;
    border-radius: 8px;
    min-height: 32px;
    min-width: 136px;
    max-width: 220px;
    margin: 0 2px;
    color: #8c90a4;
    transition: background 120ms ease, color 120ms ease, border-color 120ms ease;
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
    border-radius: 8px;
    padding: 4px 4px 4px 10px;
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
    border-radius: 6px;
    min-width: 22px;
    min-height: 22px;
    padding: 0;
    margin: 4px 5px 4px 2px;
    color: inherit;
    opacity: 0.35;
    transition: opacity 100ms ease, background 100ms ease;
}
.kestrel-tab:hover .kestrel-tab-close,
.kestrel-tab.active .kestrel-tab-close {
    opacity: 0.75;
}
.kestrel-tab-close:hover {
    background: rgba(255, 255, 255, 0.10);
    opacity: 1;
}

.kestrel-new-tab-button {
    background: transparent;
    background-image: none;
    border: 1px solid transparent;
    box-shadow: none;
    border-radius: 8px;
    min-width: 32px;
    min-height: 32px;
    padding: 0;
    margin: 0 2px 0 4px;
    color: #c8cad4;
    transition: background 100ms ease;
}
.kestrel-new-tab-button:hover {
    background: rgba(255, 255, 255, 0.07);
}
window.theme-light .kestrel-new-tab-button:hover {
    background: rgba(0, 0, 0, 0.07);
}

/* ============================================================
   TOOLBAR
   ============================================================ */
.kestrel-toolbar {
    background: #161820;
    padding: 6px 10px;
    border-bottom: 1px solid #222532;
    min-height: 46px;
}
window.theme-light .kestrel-toolbar {
    background: #f5f6f8;
    border-bottom: 1px solid #d8dce6;
}

.kestrel-toolbar-button {
    background: transparent;
    background-image: none;
    border: 1px solid transparent;
    box-shadow: none;
    border-radius: 8px;
    min-width: 34px;
    min-height: 34px;
    padding: 0 6px;
    margin: 0 1px;
    color: #d4d6e0;
    transition: background 100ms ease;
}
.kestrel-toolbar-button:hover {
    background: rgba(255, 255, 255, 0.07);
}
.kestrel-toolbar-button:active,
.kestrel-toolbar-button:checked {
    background: rgba(255, 255, 255, 0.11);
}
.kestrel-toolbar-button:disabled {
    opacity: 0.35;
}
window.theme-light .kestrel-toolbar-button:hover {
    background: rgba(0, 0, 0, 0.06);
}

.kestrel-toolbar-button > arrow,
.kestrel-shield-button > arrow {
    -gtk-icon-source: none;
    min-width: 0;
    min-height: 0;
    padding: 0;
    margin: 0;
}

/* Nút Shields đơn sắc tinh tế */
.kestrel-shield-button {
    background: rgba(255, 255, 255, 0.05);
    background-image: none;
    border: 1px solid #2b2e3d;
    box-shadow: none;
    border-radius: 17px;
    min-height: 32px;
    padding: 0 10px;
    margin: 0 3px;
    transition: background 120ms ease, border-color 120ms ease;
}
.kestrel-shield-button:hover {
    background: rgba(255, 255, 255, 0.09);
    border-color: #3a3e52;
}
.kestrel-shield-button.shield-off {
    background: transparent;
    border-color: transparent;
    opacity: 0.65;
}
.kestrel-shield-count {
    font-size: 11.5px;
    font-weight: 600;
    color: #d4d6e0;
    margin-left: 4px;
}
.kestrel-shield-button.shield-off .kestrel-shield-count {
    color: #767a8c;
}
window.theme-light .kestrel-shield-button {
    background: rgba(0, 0, 0, 0.05);
    border-color: #cfd4df;
}
window.theme-light .kestrel-shield-count {
    color: #222530;
}

/* ============================================================
   COMPOSITE OMNIBOX
   ============================================================ */
.kestrel-omnibox-box {
    background: #0f1117;
    border: 1px solid #262936;
    border-radius: 19px;
    padding: 2px 6px 2px 10px;
    min-height: 34px;
    margin: 0 8px;
    transition: background 150ms ease, border-color 150ms ease, box-shadow 150ms ease;
}
.kestrel-omnibox-box:focus-within {
    background: #12141c;
    border-color: #525870;
    box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.06);
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
    padding: 0 6px 0 2px;
    opacity: 0.85;
}

.kestrel-omnibox-scheme-badge {
    font-size: 11px;
    font-weight: 500;
    color: #8c90a4;
    padding-right: 4px;
}

.kestrel-omnibox-entry {
    background: transparent;
    background-image: none;
    color: #eceef5;
    border: none;
    box-shadow: none;
    outline: none;
    padding: 4px 6px;
    min-height: 24px;
    font-size: 13.5px;
    caret-color: #e4e6ef;
}
.kestrel-omnibox-entry:focus {
    background: transparent;
    border: none;
    box-shadow: none;
}
.kestrel-omnibox-entry selection {
    background: #3b4054;
    color: #ffffff;
}
window.theme-light .kestrel-omnibox-entry {
    color: #151820;
    caret-color: #151820;
}

.kestrel-omnibox-star {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 14px;
    min-width: 28px;
    min-height: 28px;
    padding: 0;
    margin-left: 2px;
}
.kestrel-omnibox-star:hover {
    background: rgba(255, 255, 255, 0.07);
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
    border-radius: 0;
}
progressbar.kestrel-progress progress {
    min-height: 2px;
    background: #c8cad4;
    border: none;
    border-radius: 0;
}

/* ============================================================
   CONTENT AREA
   ============================================================ */
notebook.kestrel-content {
    background: #111319;
    border: none;
}

/* ============================================================
   MENU POPOVER
   ============================================================ */
popover > contents {
    background: #181a22;
    border-radius: 12px;
    padding: 6px;
    border: 1px solid #2a2d3c;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.5);
}
window.theme-light popover > contents {
    background: #ffffff;
    border: 1px solid #d4d8e2;
    box-shadow: 0 10px 28px rgba(0, 0, 0, 0.14);
}

.kestrel-menu {
    background: transparent;
    padding: 4px;
    min-width: 225px;
}

.kestrel-shield-popover {
    min-width: 270px;
    padding: 6px;
}

.kestrel-shield-header {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid #282b3a;
    border-radius: 9px;
    padding: 10px 12px;
    margin-bottom: 8px;
}

.kestrel-shield-stat-num {
    font-size: 18px;
    font-weight: 700;
    color: #f2f4fa;
}

.kestrel-shield-stat-label {
    font-size: 11.5px;
    color: #8c90a4;
}

.kestrel-menu-item {
    background: transparent;
    background-image: none;
    border: 1px solid transparent;
    box-shadow: none;
    border-radius: 7px;
    padding: 8px 12px;
    color: #dfe1eb;
    font-size: 13px;
    font-weight: 500;
    transition: background 100ms ease, color 100ms ease;
}
.kestrel-menu-item:hover {
    background: rgba(255, 255, 255, 0.07);
    color: #ffffff;
}
.kestrel-menu-item:checked {
    background: rgba(255, 255, 255, 0.10);
    border-color: #3a3e52;
    color: #ffffff;
}
window.theme-light .kestrel-menu-item {
    color: #1d202a;
}
window.theme-light .kestrel-menu-item:hover {
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
    letter-spacing: 1px;
    color: #858a9e;
    padding: 6px 12px 4px 12px;
}

.kestrel-menu-desc {
    font-size: 11px;
    color: #8c91a6;
    padding: 0 12px 8px 12px;
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
