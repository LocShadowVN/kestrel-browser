mod app;
mod chrome;
mod engine;
mod storage;
mod util;

fn main() {
    // Kestrel sử dụng servo-gtk, chạy Servo trong subprocess.
    // PHẢI gọi dòng này ĐẦU TIÊN trong main().
    servo_gtk::run_as_runner_if_requested();

    util::log::init();

    log::info!("Kestrel Browser starting (Servo engine)");

    let app = app::KestrelApp::new();
    app.run();
}
