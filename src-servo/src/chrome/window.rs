use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Notebook, Orientation,
};
use std::rc::Rc;

use crate::chrome::toolbar::Toolbar;
use crate::engine::browser::BrowserState;
use crate::storage::database::Database;

pub struct MainWindow {
    window: ApplicationWindow,
    browser: Rc<BrowserState>,
}

impl MainWindow {
    pub fn new(app: &Application, db: Rc<Database>) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Kestrel Browser")
            .default_width(1400)
            .default_height(900)
            .build();

        let browser = Rc::new(BrowserState::new(db));

        // Toolbar (back/forward/reload + omnibox)
        let toolbar = Toolbar::new(browser.clone());

        // Tab bar
        let notebook = Notebook::new();
        notebook.set_vexpand(true);
        notebook.set_hexpand(true);

        // Kết nối notebook với browser
        let browser_notebook = browser.clone();
        let nb = notebook.clone();
        notebook.connect_switch_page(move |_, _, page_num| {
            browser_notebook.set_active_tab(page_num as usize);
        });

        // Thêm tab đầu tiên
        let browser_init = browser.clone();
        let nb_init = notebook.clone();
        glib::idle_add_local_once(move || {
            browser_init.open_tab_in_notebook(&nb_init, "about:blank");
        });

        // Layout
        let vbox = GtkBox::new(Orientation::Vertical, 0);
        vbox.append(toolbar.widget());
        vbox.append(&notebook);

        window.set_child(Some(&vbox));

        Self { window, browser }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
