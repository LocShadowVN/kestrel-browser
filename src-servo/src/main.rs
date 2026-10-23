mod adblock_rules;
mod app;
mod chrome;
mod compat;
mod engine;
mod internal_pages;
mod storage;
mod util;

use std::env;
use std::fs;
use std::path::Path;

fn should_enable_dri_prime() -> bool {
    let card0_vendor = fs::read_to_string("/sys/class/drm/card0/device/vendor")
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    let card1_vendor = fs::read_to_string("/sys/class/drm/card1/device/vendor")
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();

    card0_vendor == "0x8086" && (card1_vendor == "0x1002" || card1_vendor == "0x10de")
}

/// Thiết lập biến môi trường GPU phần cứng và bộ đệm Shader
/// mà KHÔNG tắt VSync để tránh hiện tượng tiến trình con làm ngập ống IPC của GTK4.
fn init_gpu_acceleration_env() {
    if env::var_os("LIBGL_ALWAYS_SOFTWARE").is_none() {
        env::set_var("LIBGL_ALWAYS_SOFTWARE", "0");
    }

    let has_nvidia = Path::new("/proc/driver/nvidia/version").exists()
        || Path::new("/dev/nvidiactl").exists();

    if has_nvidia {
        if env::var_os("__NV_PRIME_RENDER_OFFLOAD").is_none() {
            env::set_var("__NV_PRIME_RENDER_OFFLOAD", "1");
        }
        if env::var_os("__VK_LAYER_NV_optimus").is_none() {
            env::set_var("__VK_LAYER_NV_optimus", "NVIDIA_only");
        }
        if env::var_os("__GL_THREADED_OPTIMIZATIONS").is_none() {
            env::set_var("__GL_THREADED_OPTIMIZATIONS", "1");
        }
        if env::var_os("__GL_SHADER_DISK_CACHE").is_none() {
            env::set_var("__GL_SHADER_DISK_CACHE", "1");
        }
        if env::var_os("__GL_SHADER_DISK_CACHE_SKIP_CLEANUP").is_none() {
            env::set_var("__GL_SHADER_DISK_CACHE_SKIP_CLEANUP", "1");
        }
    } else {
        if env::var_os("DRI_PRIME").is_none() && should_enable_dri_prime() {
            env::set_var("DRI_PRIME", "1");
        }
        if env::var_os("mesa_glthread").is_none() {
            env::set_var("mesa_glthread", "true");
        }
        if env::var_os("MESA_SHADER_CACHE_DISABLE").is_none() {
            env::set_var("MESA_SHADER_CACHE_DISABLE", "false");
        }
    }
}

fn main() -> glib::ExitCode {
    init_gpu_acceleration_env();

    // CRITICAL: servo-gtk spawns a Servo runner subprocess by re-executing this binary.
    servo_gtk::run_as_runner_if_requested();

    util::log::init();

    log::info!("Kestrel Browser starting (Servo engine + Hardware GPU Acceleration)");

    std::thread::spawn(|| {
        let blob = adblock_rules::get_blob();
        if blob.is_empty() {
            log::warn!("Adblock blob empty; network adblock will be unavailable");
        } else {
            log::info!("Adblock blob ready ({} KB)", blob.len() / 1024);
        }
    });

    let app = app::KestrelApp::new();
    app.run()
}
