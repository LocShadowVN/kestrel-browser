//! Global CSS cho Kestrel — Thiết kế hiện đại lấy cảm hứng từ Brave Browser.
//! Hỗ trợ cả Dark Theme (mặc định) và Light Theme (`.theme-light`).

use gtk4::prelude::*;

const CSS: &str = r#"
/* ============================================================
   WINDOW & THEME PALETTE
   ============================================================ */
window.kestrel-window {
    background-color: #0b0c10;
    color: #eceef4;
}

window.kestrel-window.theme-light {
    background-color: #e8eaef;
    color: #181a20;
}

/* ============================================================
   TAB STRIP — Thanh tab hiện đại kiểu Brave
   ============================================================ */
.kestrel-tab-strip {
    background: linear-gradient(180deg, #0b0c10 0%, #101218 100%);
    padding: 6px 8px 4px 8px;
    min-height: 40px;
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

/* Mỗi tab: floating pill bo tròn 9px kiểu Brave mới */
.kestrel-tab {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid transparent;
    border-radius: 9px;
    min-height: 32px;
    min-width: 136px;
    max-width: 220px;
    margin: 0 2px;
    color: #9499ad;
    transition: background 120ms ease, color 120ms ease, border-color 120ms ease;
}
.kestrel-tab:hover {
    background: rgba(255, 255, 255, 0.075);
    color: #d8dae6;
}
.kestrel-tab.active {
    background: #1c1e28;
    color: #ffffff;
    border-color: #2d3142;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.35), inset 0 -2px 0 0 #ff622d;
}

window.theme-light .kestrel-tab {
    background: rgba(0, 0, 0, 0.04);
    color: #525866;
}
window.theme-light .kestrel-tab:hover {
    background: rgba(0, 0, 0, 0.08);
    color: #1e2028;
}
window.theme-light .kestrel-tab.active {
    background: #ffffff;
    color: #111318;
    border-color: #c9ced8;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.08), inset 0 -2px 0 0 #ff622d;
}

/* Thân tab: Button trong suốt */
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
    opacity: 0.85;
}
.kestrel-tab.active .kestrel-tab-favicon {
    opacity: 1;
}

.kestrel-tab-label {
    font-size: 12.5px;
    color: inherit;
}

/* Nút X đóng tab */
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
    opacity: 0.8;
}
.kestrel-tab-close:hover {
    background: rgba(255, 90, 90, 0.22);
    opacity: 1;
}

/* Nút + thêm tab */
.kestrel-new-tab-button {
    background: rgba(255, 255, 255, 0.04);
    background-image: none;
    border: 1px solid transparent;
    box-shadow: none;
    border-radius: 8px;
    min-width: 32px;
    min-height: 32px;
    padding: 0;
    margin: 0 2px 0 6px;
    color: #c5c8d6;
    transition: background 100ms ease, border-color 100ms ease;
}
.kestrel-new-tab-button:hover {
    background: #232634;
    border-color: #32364a;
}
window.theme-light .kestrel-new-tab-button {
    background: rgba(0, 0, 0, 0.05);
}
window.theme-light .kestrel-new-tab-button:hover {
    background: #ffffff;
    border-color: #c5cad4;
}

/* ============================================================
   TOOLBAR — Thanh điều hướng & địa chỉ
   ============================================================ */
.kestrel-toolbar {
    background: #161821;
    padding: 6px 10px;
    border-bottom: 1px solid #222533;
    min-height: 46px;
}
window.theme-light .kestrel-toolbar {
    background: #f5f6f8;
    border-bottom: 1px solid #d8dce6;
}

/* Nút toolbar: 34x34, bo góc 8px */
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
    color: #d8dae6;
    transition: background 100ms ease, border-color 100ms ease;
}
.kestrel-toolbar-button:hover {
    background: #252836;
    border-color: #32364a;
}
.kestrel-toolbar-button:active,
.kestrel-toolbar-button:checked {
    background: #2d3142;
}
.kestrel-toolbar-button:disabled {
    opacity: 0.38;
}
window.theme-light .kestrel-toolbar-button:hover {
    background: #e4e7ee;
    border-color: #cfd4df;
}

/* Ẩn arrow mặc định của MenuButton */
.kestrel-toolbar-button > arrow,
.kestrel-shield-button > arrow {
    -gtk-icon-source: none;
    min-width: 0;
    min-height: 0;
    padding: 0;
    margin: 0;
}

/* Nút Shields có badge số lượng quảng cáo đã chặn */
.kestrel-shield-button {
    background: rgba(255, 98, 45, 0.10);
    background-image: none;
    border: 1px solid rgba(255, 98, 45, 0.25);
    box-shadow: none;
    border-radius: 18px;
    min-height: 32px;
    padding: 0 10px;
    margin: 0 3px;
    transition: background 120ms ease, border-color 120ms ease;
}
.kestrel-shield-button:hover {
    background: rgba(255, 98, 45, 0.18);
    border-color: rgba(255, 98, 45, 0.45);
}
.kestrel-shield-button.shield-off {
    background: rgba(255, 255, 255, 0.04);
    border-color: rgba(255, 255, 255, 0.08);
}
.kestrel-shield-count {
    font-size: 11.5px;
    font-weight: 700;
    color: #ff7a4d;
    margin-left: 4px;
}
.kestrel-shield-button.shield-off .kestrel-shield-count {
    color: #7c8094;
}

/* ============================================================
   COMPOSITE OMNIBOX — Pill shape tích hợp khóa HTTPS + Star
   ============================================================ */
.kestrel-omnibox-box {
    background: #0e1017;
    border: 1px solid #282c3e;
    border-radius: 20px;
    padding: 2px 6px 2px 10px;
    min-height: 34px;
    margin: 0 8px;
    transition: background 150ms ease, border-color 150ms ease, box-shadow 150ms ease;
}
.kestrel-omnibox-box:focus-within {
    background: #12151f;
    border-color: #ff622d;
    box-shadow: 0 0 0 2px rgba(255, 98, 45, 0.20);
}
window.theme-light .kestrel-omnibox-box {
    background: #ffffff;
    border-color: #cdd2de;
}
window.theme-light .kestrel-omnibox-box:focus-within {
    background: #ffffff;
    border-color: #ff622d;
}

.kestrel-omnibox-security {
    padding: 0 6px 0 2px;
    opacity: 0.92;
}

.kestrel-omnibox-scheme-badge {
    font-size: 11px;
    font-weight: 600;
    color: #8e93a6;
    padding-right: 4px;
}

.kestrel-omnibox-entry {
    background: transparent;
    background-image: none;
    color: #f2f4fa;
    border: none;
    box-shadow: none;
    outline: none;
    padding: 4px 6px;
    min-height: 24px;
    font-size: 13.5px;
    caret-color: #ff622d;
}
.kestrel-omnibox-entry:focus {
    background: transparent;
    border: none;
    box-shadow: none;
}
.kestrel-omnibox-entry selection {
    background: #ff622d;
    color: #ffffff;
}
window.theme-light .kestrel-omnibox-entry {
    color: #151820;
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
    background: rgba(255, 255, 255, 0.08);
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
    background: rgba(255, 98, 45, 0.12);
    border: none;
    border-radius: 0;
}
progressbar.kestrel-progress progress {
    min-height: 2px;
    background: linear-gradient(90deg, #ff622d 0%, #5e9eff 100%);
    border: none;
    border-radius: 0;
}

/* ============================================================
   CONTENT AREA
   ============================================================ */
notebook.kestrel-content {
    background: #12131a;
    border: none;
}

/* ============================================================
   MENU POPOVER — Hamburger + Kestrel Shields Popover
   ============================================================ */
popover > contents {
    background: #1a1d28;
    border-radius: 14px;
    padding: 6px;
    border: 1px solid #2d3246;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.55);
}
window.theme-light popover > contents {
    background: #ffffff;
    border: 1px solid #d4d8e2;
    box-shadow: 0 10px 28px rgba(0, 0, 0, 0.15);
}

.kestrel-menu {
    background: transparent;
    padding: 4px;
    min-width: 230px;
}

.kestrel-shield-popover {
    min-width: 280px;
    padding: 6px;
}

.kestrel-shield-header {
    background: rgba(255, 98, 45, 0.09);
    border: 1px solid rgba(255, 98, 45, 0.22);
    border-radius: 10px;
    padding: 10px 12px;
    margin-bottom: 8px;
}

.kestrel-shield-stat-num {
    font-size: 20px;
    font-weight: 800;
    color: #ff7a4d;
}

.kestrel-shield-stat-label {
    font-size: 11.5px;
    color: #9ea3b8;
}

.kestrel-menu-item {
    background: transparent;
    background-image: none;
    border: 1px solid transparent;
    box-shadow: none;
    border-radius: 8px;
    padding: 8px 12px;
    color: #e4e6f0;
    font-size: 13px;
    font-weight: 500;
    transition: background 100ms ease, color 100ms ease;
}
.kestrel-menu-item:hover {
    background: #262a3a;
    color: #ffffff;
}
.kestrel-menu-item:checked {
    background: rgba(255, 98, 45, 0.16);
    border-color: rgba(255, 98, 45, 0.45);
    color: #ff855c;
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
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 1.1px;
    color: #858a9e;
    padding: 6px 12px 4px 12px;
}

.kestrel-menu-desc {
    font-size: 11px;
    color: #8c91a6;
    padding: 0 12px 8px 12px;
}

.kestrel-menu-separator {
    background: #2b2f42;
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
