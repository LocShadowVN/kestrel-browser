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

/// Kiểm tra xem DRM card có phải GPU rời (AMD/NVIDIA) đi kèm iGPU Intel ở card0 hay không.
fn should_enable_dri_prime() -> bool {
    let card0_vendor = fs::read_to_string("/sys/class/drm/card0/device/vendor")
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    let card1_vendor = fs::read_to_string("/sys/class/drm/card1/device/vendor")
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();

    // Chỉ tự động bật DRI_PRIME=1 khi card0 là Intel iGPU (0x8086)
    // và card1 là AMD (0x1002) hoặc NVIDIA (0x10de).
    card0_vendor == "0x8086" && (card1_vendor == "0x1002" || card1_vendor == "0x10de")
}

/// Thiết lập biến môi trường tối ưu hóa GPU phần cứng và OpenGL/WebRender
/// TRƯỚC KHI tiến trình con Servo runner được tạo.
fn init_gpu_acceleration_env() {
    // 1. Ưu tiên phần cứng nếu người dùng không chủ động cấu hình LIBGL_ALWAYS_SOFTWARE
    if env::var_os("LIBGL_ALWAYS_SOFTWARE").is_none() {
        env::set_var("LIBGL_ALWAYS_SOFTWARE", "0");
    }

    // 2. Ép GTK4 dùng OpenGL hardware renderer đồng bộ với GLArea của servo-gtk
    if env::var_os("GSK_RENDERER").is_none() {
        env::set_var("GSK_RENDERER", "gl");
    }

    // 3. Tắt chờ VBlank trong offscreen buffer của tiến trình con Servo để tránh Double-VSync
    if env::var_os("vblank_mode").is_none() {
        env::set_var("vblank_mode", "0");
    }
    if env::var_os("__GL_SYNC_TO_VBLANK").is_none() {
        env::set_var("__GL_SYNC_TO_VBLANK", "0");
    }

    // 4. Tự động phát hiện và tối ưu hóa cho NVIDIA hoặc Mesa (AMD / Intel)
    let has_nvidia = Path::new("/proc/driver/nvidia/version").exists()
        || Path::new("/dev/nvidiactl").exists();

    if has_nvidia {
        if env::var_os("__NV_PRIME_RENDER_OFFLOAD").is_none() {
            env::set_var("__NV_PRIME_RENDER_OFFLOAD", "1");
        }
        if env::var_os("__VK_LAYER_NV_optimus").is_none() {
            env::set_var("__VK_LAYER_NV_optimus", "NVIDIA_only");
        }
        if env::var_os("__GLX_VENDOR_LIBRARY_NAME").is_none() {
            env::set_var("__GLX_VENDOR_LIBRARY_NAME", "nvidia");
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

    // Khởi tạo sớm adblock blob trên luồng nền ngay trước khi GTK khởi chạy
    std::thread::spawn(|| {
        let blob = adblock_rules::get_blob();
        if blob.is_empty() {
            log::warn!("Adblock blob empty; adblock will be unavailable");
        } else {
            log::info!("Adblock blob ready ({} KB)", blob.len() / 1024);
        }
    });

    let app = app::KestrelApp::new();
    app.run()
}
