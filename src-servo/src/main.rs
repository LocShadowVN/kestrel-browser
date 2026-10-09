mod app;
mod chrome;
mod compat;
mod engine;
mod internal_pages;
mod storage;
mod util;

fn main() -> glib::ExitCode {
    // CRITICAL: servo-gtk spawns a Servo runner subprocess by re-executing
    // this binary. Phải gọi ở dòng đầu tiên, trước mọi thứ khác.
    servo_gtk::run_as_runner_if_requested();

    util::log::init();

    log::info!("Kestrel Browser starting (Servo engine)");

    let app = app::KestrelApp::new();
    app.run()
}
