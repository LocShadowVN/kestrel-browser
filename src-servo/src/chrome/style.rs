//! Global CSS cho Kestrel — phong cách Brave.

use gtk4::prelude::*;

const CSS: &str = r#"
window.kestrel-window { background-color: #1a1a1f; }

notebook.kestrel-tabbar > header {
    background: #0d0d12;
    border: none;
    padding: 6px 6px 0 6px;
    min-height: 36px;
}
notebook.kestrel-tabbar > header > tabs > tab {
    background: transparent;
    border: none;
    border-radius: 8px 8px 0 0;
    padding: 0;
    margin: 0 1px;
    min-width: 180px;
    max-width: 240px;
    min-height: 32px;
    color: #9a9aa2;
}
notebook.kestrel-tabbar > header > tabs > tab:checked {
    background: #1c1c22;
    color: #ffffff;
}
notebook.kestrel-tabbar > header > tabs > tab:hover:not(:checked) {
    background: #1a1a20;
    color: #d0d0d6;
}

.kestrel-tab-content { padding: 4px 8px; min-height: 24px; }
.kestrel-tab-favicon { color: inherit; }
.kestrel-tab-label { font-size: 12px; color: inherit; }
.kestrel-tab-close {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 6px;
    min-width: 20px;
    min-height: 20px;
    padding: 0;
    margin: 0;
    color: inherit;
    opacity: 0.6;
}
.kestrel-tab-close:hover { background: #3a3a45; opacity: 1; color: #ffffff; }

.kestrel-new-tab-button {
    background: transparent;
    background-image: none;
    border: none;
    box-shadow: none;
    border-radius: 6px;
    min-width: 30px;
    min-height: 30px;
    padding: 0;
    margin: 3px 4px;
    color: #c0c0c8;
}
.kestrel-new-tab-button:hover { background: #2f2f38; color: #ffffff; }

.kestrel-toolbar {
    background: #1c1c22;
    padding: 6px 10px;
    border-bottom: 1px solid #0a0a0f;
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
