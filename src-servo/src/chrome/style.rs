// src-servo/src/chrome/style.rs
//! Global CSS cho Kestrel — Ngôn ngữ thiết kế Monochrome Carbon lấy cảm hứng từ Brave,
//! sắc nét, chuyên nghiệp, không dùng màu sắc lòe loẹt, tương phản chuẩn cả Dark lẫn Light Mode.

const CSS: &str = r#"
/* ============================================================
   WINDOW & THEME PALETTE
   ============================================================ */
window.kestrel-window {
    background-color: #0d0f14;
    color: #e6e8f0;
}

window.kestrel-window.theme-light {
    background-color: #edeff5;
    color: #16181f;
}

/* Chuyển các icon SVG đơn sắc sáng sang màu than chì sắc nét trong Light Mode */
window.theme-light .kestrel-tab-strip image,
window.theme-light .kestrel-toolbar button.kestrel-toolbar-button image,
window.theme-light .kestrel-toolbar menubutton.kestrel-toolbar-button image,
window.theme-light .kestrel-omnibox-security,
window.theme-light .kestrel-omnibox-shield image,
window.theme-light .kestrel-find-bar image,
window.theme-light popover image {
    filter: brightness(0.22);
}

window.theme-light button.kestrel-omnibox-action:not(.bookmarked) image {
    filter: brightness(0.26);
}

window.theme-light button.kestrel-omnibox-action.bookmarked image {
    filter: brightness(0.16);
}

/* ============================================================
   TAB STRIP (BRAVE ARCHITECTURE)
   ============================================================ */
.kestrel-tab-strip {
    background: #0b0d12;
    padding: 5px 8px 4px 8px;
    min-height: 38px;
    border-bottom: 1px solid #1a1d27;
}
window.theme-light .kestrel-tab-strip {
    background: #dde0e9;
    border-bottom: 1px solid #c8ccd8;
}

.kestrel-tabs-scroll {
    background: transparent;
    border: none;
}
.kestrel-tabs-scroll scrollbar {
    min-height: 0;
    min-width: 0;
    opacity: 0;
    background: transparent;
}

.kestrel-tabs-area {
    min-height: 30px;
}

.kestrel-tab {
    background: transparent;
    border: 1px solid transparent;
    border-radius: 8px;
    min-height: 30px;
    min-width: 140px;
    max-width: 224px;
    margin: 0 2px;
    color: #8a8fa3;
    transition: background-color 100ms ease, border-color 100ms ease, color 100ms ease;
}
.kestrel-tab:hover {
    background: rgba(255, 255, 255, 0.05);
    color: #dcdfe8;
}
.kestrel-tab.active {
    background: #171a24;
    color: #f4f6fc;
    border-color: #292e3e;
}

window.theme-light .kestrel-tab {
    background: transparent;
    color: #505668;
}
window.theme-light .kestrel-tab:hover {
    background: rgba(0, 0, 0, 0.055);
    color: #181b22;
}
window.theme-light .kestrel-tab.active {
    background: #ffffff;
    color: #111318;
    border-color: #c2c7d4;
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
    opacity: 0.76;
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
    opacity: 0.42;
    transition: background-color 90ms ease, opacity 90ms ease;
}
.kestrel-tab:hover .kestrel-tab-close,
.kestrel-tab.active .kestrel-tab-close {
    opacity: 0.84;
}
.kestrel-tab-close:hover {
    background: rgba(255, 255, 255, 0.14);
    opacity: 1;
}
window.theme-light .kestrel-tab-close:hover {
    background: rgba(0, 0, 0, 0.1);
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
    transition: background-color 100ms ease;
}
.kestrel-new-tab-button:hover {
    background: rgba(255, 255, 255, 0.08);
}
window.theme-light .kestrel-new-tab-button:hover {
    background: rgba(0, 0, 0, 0.07);
}

/* ============================================================
   NAVIGATION TOOLBAR
   ============================================================ */
.kestrel-toolbar {
    background: #151821;
    padding: 5px 10px;
    border-bottom: 1px solid #222634;
    min-height: 44px;
}
window.theme-light .kestrel-toolbar {
    background: #f6f7fa;
    border-bottom: 1px solid #d4d8e3;
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
    color: #d8dae4;
    transition: background-color 90ms ease, opacity 90ms ease;
}
button.kestrel-toolbar-button:disabled {
    opacity: 0.32;
}
button.kestrel-toolbar-button:hover:not(:disabled),
menubutton.kestrel-toolbar-button > button:hover {
    background: rgba(255, 255, 255, 0.08);
}
button.kestrel-toolbar-button:active:not(:disabled),
button.kestrel-toolbar-button:checked,
menubutton.kestrel-toolbar-button > button:active,
menubutton.kestrel-toolbar-button > button:checked {
    background: rgba(255, 255, 255, 0.13);
}
window.theme-light button.kestrel-toolbar-button:hover:not(:disabled),
window.theme-light menubutton.kestrel-toolbar-button > button:hover {
    background: rgba(0, 0, 0, 0.07);
}
window.theme-light button.kestrel-toolbar-button:active:not(:disabled),
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
    background: #0e1016;
    border: 1px solid #262a38;
    border-radius: 17px;
    padding: 1px 6px 1px 11px;
    min-height: 34px;
    margin: 0 8px;
    transition: border-color 110ms ease, background-color 110ms ease;
}
.kestrel-omnibox-box:focus-within {
    background: #10131b;
    border-color: #545b78;
}
window.theme-light .kestrel-omnibox-box {
    background: #ffffff;
    border-color: #c7cddb;
}
window.theme-light .kestrel-omnibox-box:focus-within {
    background: #ffffff;
    border-color: #666d85;
}

.kestrel-omnibox-security {
    padding: 0 5px 0 1px;
    opacity: 0.88;
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
    caret-color: #e6e8f0;
}
entry.kestrel-omnibox-entry > text > selection {
    background: #3a4158;
    color: #ffffff;
}
window.theme-light entry.kestrel-omnibox-entry,
window.theme-light entry.kestrel-omnibox-entry > text {
    color: #15171e;
    caret-color: #15171e;
}
window.theme-light entry.kestrel-omnibox-entry > text > selection {
    background: #d0e1ff;
    color: #101218;
}

button.kestrel-omnibox-zoom {
    background: rgba(255, 255, 255, 0.075);
    background-image: none;
    border: 1px solid #2e3344;
    border-radius: 10px;
    padding: 0 7px;
    min-height: 22px;
    font-size: 11px;
    font-weight: 600;
    color: #d8dae4;
}
window.theme-light button.kestrel-omnibox-zoom {
    background: #edf0f6;
    border-color: #c7cddb;
    color: #222632;
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
    transition: background-color 90ms ease;
}
button.kestrel-omnibox-action:hover {
    background: rgba(255, 255, 255, 0.08);
}
window.theme-light button.kestrel-omnibox-action:hover {
    background: rgba(0, 0, 0, 0.07);
}

.kestrel-omnibox-divider {
    background: #252938;
    min-width: 1px;
    margin: 6px 3px;
}
window.theme-light .kestrel-omnibox-divider {
    background: #d5d9e4;
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
    transition: background-color 90ms ease;
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
    opacity: 0.52;
}

.kestrel-shield-count {
    font-size: 11.5px;
    font-weight: 600;
    color: #d8dae4;
}
window.theme-light .kestrel-shield-count {
    color: #222632;
}

/* ============================================================
   FIND IN PAGE BAR (Ctrl+F)
   ============================================================ */
.kestrel-find-bar {
    background: #13161f;
    border-bottom: 1px solid #222634;
    padding: 4px 12px;
}
window.theme-light .kestrel-find-bar {
    background: #edf0f6;
    border-bottom: 1px solid #d4d8e3;
}

entry.kestrel-find-entry,
entry.kestrel-find-entry > text {
    background: #0d0f15;
    color: #eef0f7;
    border: 1px solid #292e3e;
    border-radius: 8px;
    padding: 2px 8px;
    min-height: 26px;
    font-size: 13px;
}
window.theme-light entry.kestrel-find-entry,
window.theme-light entry.kestrel-find-entry > text {
    background: #ffffff;
    color: #15171e;
    border-color: #c7cddb;
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
    background: #d4d7e4;
    border: none;
}
window.theme-light progressbar.kestrel-progress progress {
    background: #2c303e;
}

/* ============================================================
   CONTENT AREA & POPOVER MENU
   ============================================================ */
notebook.kestrel-content {
    background: #0e1015;
    border: none;
}
window.theme-light notebook.kestrel-content {
    background: #f4f5f8;
}

popover > contents {
    background: #161922;
    border-radius: 12px;
    padding: 6px;
    border: 1px solid #292d3d;
    box-shadow: 0 10px 28px rgba(0, 0, 0, 0.48);
}
window.theme-light popover > contents {
    background: #ffffff;
    border: 1px solid #d0d4e0;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.14);
}

.kestrel-menu {
    background: transparent;
    padding: 4px;
    min-width: 226px;
}

.kestrel-shield-popover {
    min-width: 270px;
    padding: 6px;
}

.kestrel-shield-header {
    background: rgba(255, 255, 255, 0.035);
    border: 1px solid #272b3b;
    border-radius: 9px;
    padding: 10px 12px;
    margin-bottom: 8px;
}
window.theme-light .kestrel-shield-header {
    background: #f4f6fa;
    border: 1px solid #dde1eb;
}

.kestrel-shield-stat-num {
    font-size: 16.5px;
    font-weight: 700;
    color: #f2f4fa;
}
window.theme-light .kestrel-shield-stat-num {
    color: #15171e;
}

.kestrel-shield-stat-label {
    font-size: 11.5px;
    color: #8a8ea2;
}
window.theme-light .kestrel-shield-stat-label {
    color: #5a5f72;
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
    border-color: #3a3f54;
    color: #ffffff;
}
window.theme-light button.kestrel-menu-item,
window.theme-light togglebutton.kestrel-menu-item {
    color: #1b1e28;
}
window.theme-light button.kestrel-menu-item:hover,
window.theme-light togglebutton.kestrel-menu-item:hover {
    background: #edf0f6;
    color: #000000;
}
window.theme-light togglebutton.kestrel-menu-item:checked {
    background: #e4e7f0;
    border-color: #b6bdd0;
    color: #101218;
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
    color: #84899e;
    padding: 6px 11px 4px 11px;
}

.kestrel-menu-desc {
    font-size: 11px;
    color: #8a8fa4;
    padding: 0 11px 8px 11px;
}

.kestrel-menu-separator {
    background: #262938;
    min-height: 1px;
    margin: 5px 8px;
}
window.theme-light .kestrel-menu-separator {
    background: #e0e3ec;
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
