//! Main window — layout: toolbar trên, tabbar dưới.

use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation};

use crate::chrome::tabbar::TabBar;
use crate::chrome::toolbar::Toolbar;
use crate::engine::browser::BrowserState;
use crate::storage::database::Database;
use crate::util::config::Config;

pub struct MainWindow {
    window: ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &Application, db: Rc<Database>) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Kestrel Browser")
            .default_width(1400)
            .default_height(900)
            .build();

        let config = Rc::new(Config::new(db.clone()));
        let browser = Rc::new(BrowserState::new(db, config));

        let toolbar = Rc::new(Toolbar::new(browser.clone()));
        let tabbar = Rc::new(TabBar::new());

        // Kết nối tabbar → browser
        let browser_switch = browser.clone();
        tabbar.set_on_switch(move |index| {
            browser_switch.set_active_tab(index);
        });

        // Layout
        let vbox = GtkBox::new(Orientation::Vertical, 0);
        vbox.append(toolbar.widget());
        vbox.append(tabbar.widget());

        window.set_child(Some(&vbox));

        // Mở tab đầu tiên sau khi window ready
        let browser_init = browser.clone();
        let tabbar_init = tabbar.clone();
        let toolbar_init = toolbar.clone();
        let window_init = window.clone();
        window_init.connect_map(move |_| {
            if tabbar_init.count() == 0 {
                let homepage = browser_init.homepage();
                browser_init.open_tab(&tabbar_init, &homepage);
                toolbar_init.set_url(&homepage);
            }
        });

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
