use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::Application;

use crate::chrome::style;
use crate::chrome::window::MainWindow;
use crate::storage::database::Database;
use crate::util::config::Config;

pub struct KestrelApp {
    db: Rc<Database>,
    config: Rc<Config>,
}

impl KestrelApp {
    pub fn new() -> Self {
        let db = Rc::new(Database::init());
        let config = Rc::new(Config::new(db.clone()));
        Self { db, config }
    }

    pub fn run(&self) -> glib::ExitCode {
        let app = Application::builder()
            .application_id("io.github.locshadowvn.Kestrel")
            .build();

        let db = self.db.clone();
        let config = self.config.clone();

        app.connect_startup(|_app| {
            style::load();
        });

        app.connect_activate(move |app| {
            let window = MainWindow::new(app, db.clone(), config.clone());
            window.present();
        });

        app.run()
    }
}
