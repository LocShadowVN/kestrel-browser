mod adblock_rules;
mod app;
mod chrome;
mod compat;
mod engine;
mod internal_pages;
mod storage;
mod util;

use std::env;
use std::path::Path;

/// Thiết lập biến môi trường ép sử dụng GPU phần cứng / Card rời (NVIDIA & AMD/Mesa)
/// và tối ưu hóa OpenGL/WebRender TRƯỚC KHI tiến trình con Servo runner được tạo.
fn init_gpu_acceleration_env() {
    // 1. Tuyệt đối không dùng CPU software rasterizer (llvmpipe/softpipe)
    env::set_var("LIBGL_ALWAYS_SOFTWARE", "0");

    // 2. Ép GTK4 dùng OpenGL hardware renderer đồng bộ với GLArea của servo-gtk
    if env::var_os("GSK_RENDERER").is_none() {
        env::set_var("GSK_RENDERER", "gl");
    }

    // 3. Tắt chờ VBlank trong offscreen buffer của tiến trình con Servo để tránh Double-VSync gây giật
    if env::var_os("vblank_mode").is_none() {
        env::set_var("vblank_mode", "0");
    }
    if env::var_os("__GL_SYNC_TO_VBLANK").is_none() {
        env::set_var("__GL_SYNC_TO_VBLANK", "0");
    }

    // 4. Tự động phát hiện và ép chạy trên Card rời (Discrete GPU)
    let has_nvidia = Path::new("/proc/driver/nvidia/version").exists()
        || Path::new("/dev/nvidiactl").exists();

    if has_nvidia {
        // Hệ thống có driver NVIDIA rời (Optimus / Prime Render Offload)
        if env::var_os("__NV_PRIME_RENDER_OFFLOAD").is_none() {
            env::set_var("__NV_PRIME_RENDER_OFFLOAD", "1");
        }
        if env::var_os("__VK_LAYER_NV_optimus").is_none() {
            env::set_var("__VK_LAYER_NV_optimus", "NVIDIA_only");
        }
        if env::var_os("__GLX_VENDOR_LIBRARY_NAME").is_none() {
            env::set_var("__GLX_VENDOR_LIBRARY_NAME", "nvidia");
        }
        // Bật đa luồng OpenGL và bộ nhớ đệm Shader trên đĩa cho NVIDIA
        env::set_var("__GL_THREADED_OPTIMIZATIONS", "1");
        env::set_var("__GL_SHADER_DISK_CACHE", "1");
        env::set_var("__GL_SHADER_DISK_CACHE_SKIP_CLEANUP", "1");
    } else {
        // Hệ thống AMD / Intel Arc / Mesa: nếu có card thứ 2 (card1) thì ưu tiên DRI_PRIME=1
        if Path::new("/sys/class/drm/card1").exists() && env::var_os("DRI_PRIME").is_none() {
            env::set_var("DRI_PRIME", "1");
        }
        // Bật OpenGL multithreading và shader cache cho Mesa
        if env::var_os("mesa_glthread").is_none() {
            env::set_var("mesa_glthread", "true");
        }
        env::set_var("MESA_SHADER_CACHE_DISABLE", "false");
    }
}

fn main() -> glib::ExitCode {
    // Khởi tạo biến môi trường GPU trước tiên để cả tiến trình chính lẫn tiến trình con Servo đều kế thừa
    init_gpu_acceleration_env();

    // CRITICAL: servo-gtk spawns a Servo runner subprocess by re-executing this binary.
    servo_gtk::run_as_runner_if_requested();

    util::log::init();

    log::info!("Kestrel Browser starting (Servo engine + Hardware GPU Acceleration)");

    // Nạp / biên dịch adblock blob trên luồng nền để cửa sổ trình duyệt hiện lên tức thì
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
