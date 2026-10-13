mod adblock_rules;
mod app;
mod chrome;
mod compat;
mod engine;
mod internal_pages;
mod storage;
mod util;

fn main() -> glib::ExitCode {
    // CRITICAL: servo-gtk spawns a Servo runner subprocess by re-executing
    // this binary. Phải gọi ở dòng đầu tiên.
    servo_gtk::run_as_runner_if_requested();

    util::log::init();

    log::info!("Kestrel Browser starting (Servo engine)");

    // Warm adblock blob trước khi mở app: build lần đầu (~10s) hoặc load cache.
    let blob = adblock_rules::get_blob();
    if blob.is_empty() {
        log::warn!("Adblock blob empty; adblock will be unavailable");
    } else {
        log::info!("Adblock blob ready ({} KB)", blob.len() / 1024);
    }

    let app = app::KestrelApp::new();
    app.run()
}
