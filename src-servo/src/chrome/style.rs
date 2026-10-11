//! Global CSS cho Kestrel — phong cách Brave.

use gtk4::prelude::*;

const CSS: &str = r#"
window.kestrel-window { background-color: #1a1a1f; }

/* ===== TAB STRIP ===== */
.kestrel-tab-strip {
    background: #0d0d12;
    padding: 3px 6px;
    min-height: 32px;
    border-bottom: 1px solid #050508;
}
.kestrel-tabs-area { min-height: 28px; }

/* Container tab — chỉ giữ background + rounded */
.kestrel-tab {
    background: #1a1a20;
    border-radius: 7px;
    min-height: 26px;
    min-width: 100px;
    margin: 0 1px;
    color: #b0b0b8;
    transition: background 0.1s;
}
.kestrel-tab:hover { background: #23232c; color: #e0e0e6; }
.kestrel-tab.active { background: #2a2a35; color: #ffffff; }

/* Thân tab — Button trong suốt, không chiếm background của container */
.kestrel-tab-body {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 0;
    padding: 4px 4px 4px 10px;
    margin: 0;
    min-width: 0;
    min-height: 0;
    color: inherit;
    font-size: 12px;
}
.kestrel-tab-body:hover { background: transparent; }
.kestrel-tab-body:active { background: transparent; }

.kestrel-tab-favicon { color: inherit; }
.kestrel-tab-label { font-size: 12px; color: inherit; }

/* Nút X */
.kestrel-tab-close {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 4px;
    min-width: 18px;
    min-height: 18px;
    padding: 0;
    margin: 4px 4px 4px 0;
    color: inherit;
    opacity: 0.5;
}
.kestrel-tab-close:hover { background: #3a3a45; opacity: 1; color: #ffffff; }

/* Nút + */
.kestrel-new-tab-button {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 6px;
    min-width: 26px;
    min-height: 26px;
    padding: 0;
    margin: 1px 4px;
    color: #c0c0c8;
}
.kestrel-new-tab-button:hover { background: #2a2a35; color: #ffffff; }

/* ===== CONTENT ===== */
notebook.kestrel-content { background: #ffffff; border: none; }

/* ===== TOOLBAR ===== */
.kestrel-toolbar {
    background: #1c1c22;
    padding: 6px 10px;
    border-bottom: 1px solid #0a0a0f;
    min-height: 44px;
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
    margin: 0 1px;
    color: #d0d0d6;
}
.kestrel-toolbar-button:hover { background: #2f2f38; color: #ffffff; }
.kestrel-toolbar-button:active { background: #3a3a45; }
.kestrel-toolbar-button:disabled { color: #5a5a65; }

/* ===== OMNIBOX ===== */
.kestrel-omnibox {
    background: #2a2a35;
    color: #ffffff;
    border: 1px solid transparent;
    border-radius: 18px;
    padding: 6px 16px;
    min-height: 26px;
    font-size: 13px;
    caret-color: #ffffff;
    margin: 0 6px;
}
.kestrel-omnibox:focus-within { background: #313140; border-color: #5b8dee; }
.kestrel-omnibox selection { background: #3a5bcf; color: #ffffff; }

/* ===== MENU ===== */
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
.kestrel-menu-item:hover { background: #2f2f3a; color: #ffffff; }
.kestrel-menu-separator { background: #2f2f3a; min-height: 1px; margin: 4px 8px; }
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
