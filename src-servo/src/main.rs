// src-servo/src/main.rs
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

/// Quét toàn bộ `/sys/class/drm/card*/device/vendor` để phát hiện hệ thống Hybrid GPU
/// (iGPU Intel/AMD đi kèm dGPU NVIDIA hoặc AMD) bất kể thứ tự đánh số card của kernel.
fn should_enable_dri_prime() -> bool {
    let Ok(entries) = fs::read_dir("/sys/class/drm") else {
        return false;
    };

    let mut has_igpu = false;
    let mut has_dgpu = false;

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if !name_str.starts_with("card") || name_str.contains('-') {
            continue;
        }

        let vendor_path = entry.path().join("device/vendor");
        if let Ok(vendor_raw) = fs::read_to_string(vendor_path) {
            let vendor = vendor_raw.trim().to_ascii_lowercase();
            match vendor.as_str() {
                "0x8086" => has_igpu = true,
                "0x10de" | "0x1002" => has_dgpu = true,
                _ => {}
            }
        }
    }

    has_igpu && has_dgpu
}

/// Thiết lập biến môi trường GPU phần cứng và bộ đệm Shader
/// trên luồng chính trước khi bất kỳ tiến trình con hay luồng nền nào được tạo.
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

    // Khởi tạo Database & Config TRƯỚC khi tạo luồng nền để:
    // 1. Thiết lập KESTREL_USER_AGENT hoàn toàn trên single-thread (tránh data race của setenv).
    // 2. Xác định chính xác adblock_level đang kích hoạt của người dùng để ưu tiên nạp/biên dịch trước.
    let app = app::KestrelApp::new();
    let preferred_level = adblock_rules::get_active_level();

    std::thread::Builder::new()
        .name("kestrel-adblock-compiler".into())
        .spawn(move || {
            let (first_level, second_level) = if preferred_level == 1 {
                (1u8, 2u8)
            } else {
                (2u8, 1u8)
            };

            let first_blob = adblock_rules::get_blob_for_level(first_level);
            if first_blob.is_empty() {
                log::warn!(
                    "Adblock level {} blob empty; network adblock may be unavailable",
                    first_level
                );
            } else {
                log::info!(
                    "Adblock primary blob (level {}) ready ({} KB)",
                    first_level,
                    first_blob.len() / 1024
                );
            }

            let second_blob = adblock_rules::get_blob_for_level(second_level);
            if !second_blob.is_empty() {
                log::info!(
                    "Adblock secondary blob (level {}) ready ({} KB)",
                    second_level,
                    second_blob.len() / 1024
                );
            }
        })
        .ok();

    app.run()
}
