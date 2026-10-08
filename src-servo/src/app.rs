use gtk4::prelude::*;
use gtk4::Application;
use std::rc::Rc;

use crate::chrome::window::MainWindow;
use crate::storage::database::Database;

pub struct KestrelApp {
    db: Rc<Database>,
}

impl KestrelApp {
    pub fn new() -> Self {
        Self {
            db: Rc::new(Database::init()),
        }
    }

    pub fn run(&self) {
        let app = Application::builder()
            .application_id("io.github.locshadowvn.Kestrel")
            .build();

        let db = self.db.clone();
        app.connect_activate(move |app| {
            MainWindow::new(app, db.clone()).present();
        });

        app.run();
    }
}
