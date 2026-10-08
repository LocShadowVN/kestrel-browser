mod app;
mod chrome;
mod engine;
mod storage;
mod util;

fn main() -> glib::ExitCode {
    // CRITICAL: servo-gtk spawns a Servo runner subprocess by re-executing
    // this binary. If we're the runner, hand off immediately. Never returns
    // in that case.
    servo_gtk::run_as_runner_if_requested();

    util::log::init();

    // CRITICAL: Servo requires epoxy (OpenGL function loader) to be loaded
    // into the process before creating any WebView. Without this, the
    // WebView will crash on first paint.
    load_epoxy();

    log::info!("Kestrel Browser starting (Servo engine)");

    let app = app::KestrelApp::new();
    app.run()
}

/// Load libepoxy into the process. Must be called exactly once, before any
/// WebView is created.
fn load_epoxy() {
    let library = unsafe {
        libloading::os::unix::Library::new("libepoxy.so.0")
    }
    .expect("Failed to load libepoxy.so.0 — install libepoxy-dev");

    epoxy::load_with(|name| {
        unsafe { library.get::<*const std::ffi::c_void>(name.as_bytes()) }
            .map(|sym| *sym)
            .unwrap_or(std::ptr::null())
    });
}
