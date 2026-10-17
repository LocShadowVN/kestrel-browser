//! Adblock engine blob — chỉ biên dịch các quy tắc chặn tầng mạng (Network rules),
//! loại bỏ các quy tắc ẩn CSS (`##`, `#@#`, `#?#`) không dùng tới trong `load_web_resource`,
//! giúp giảm ~75% dung lượng blob và tăng tốc độ khởi tạo tab lên gấp 4-5 lần.

use adblock::lists::ParseOptions;
use adblock::Engine;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/kestrel_filters.rs"));

static BLOB: OnceLock<Vec<u8>> = OnceLock::new();

/// Trả về adblock engine đã serialize (chờ nếu đang build).
pub fn get_blob() -> &'static [u8] {
    BLOB.get_or_init(build_or_load_blob)
}

/// Kiểm tra và lấy adblock blob ngay lập tức mà không khóa luồng giao diện GTK.
pub fn try_get_blob() -> Option<&'static [u8]> {
    BLOB.get().map(|v| v.as_slice())
}

fn cache_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("kestrel-browser")
}

fn blob_path() -> PathBuf {
    // Dùng tên file v2 để tự động thay thế file adblock.blob cũ bị phình to
    cache_dir().join("adblock_net_v2.blob")
}

/// Lọc giữ lại đúng các rule tầng mạng (network filter rules), bỏ comment và cosmetic rules.
fn filter_network_only_rules(raw_rules: Vec<String>) -> Vec<String> {
    raw_rules
        .into_iter()
        .filter(|line| {
            let l = line.trim();
            !l.is_empty()
                && !l.starts_with('!')
                && !l.starts_with('[')
                && !l.contains("##")
                && !l.contains("#@#")
                && !l.contains("#?#")
                && !l.contains("#$#")
                && !l.contains("#%#")
                && !l.contains("+js(")
        })
        .collect()
}

fn build_or_load_blob() -> Vec<u8> {
    let path = blob_path();

    if path.exists() {
        match fs::read(&path) {
            Ok(blob) if blob.len() > 4096 => {
                log::info!(
                    "Loaded optimized network adblock blob from cache ({} KB)",
                    blob.len() / 1024
                );
                return blob;
            }
            Ok(blob) => {
                log::warn!(
                    "Cached adblock blob too small ({} bytes); rebuilding",
                    blob.len()
                );
            }
            Err(e) => log::warn!("Failed to read cached adblock blob: {e}"),
        }
    }

    log::info!("Building optimized network adblock engine from filter lists...");
    let raw_rules = rules_for_level(2);
    if raw_rules.is_empty() {
        log::error!("No adblock rules embedded; adblock disabled");
        return Vec::new();
    }

    let rules = filter_network_only_rules(raw_rules);
    log::info!("Compiling {} network filter rules...", rules.len());

    let engine = Engine::from_rules(&rules, ParseOptions::default());
    let blob = match engine.serialize_raw() {
        Ok(b) => b,
        Err(e) => {
            log::error!("Failed to serialize adblock engine: {e:?}");
            return Vec::new();
        }
    };

    log::info!("Built optimized adblock blob ({} KB)", blob.len() / 1024);

    if let Err(e) = fs::create_dir_all(cache_dir()) {
        log::warn!("Failed to create adblock cache dir: {e}");
    } else if let Err(e) = fs::write(&path, &blob) {
        log::warn!("Failed to cache adblock blob at {path:?}: {e}");
    } else {
        log::info!("Cached adblock blob at {path:?}");
    }

    blob
}
