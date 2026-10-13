//! Adblock engine blob — build từ filter lists một lần, cache vào đĩa.
//!
//! Lần đầu chạy: build engine từ rules (~5-15s), serialize thành blob,
//! cache vào `~/.local/share/kestrel-browser/adblock.blob`.
//!
//! Các lần sau: đọc blob từ đĩa (~50ms), gửi cho runner subprocess để
//! `Engine::deserialize()` (~200ms). Không cần parse lại 180k rules.
//!
//! Blob không portable giữa các version adblock-rust khác nhau. Khi
//! update `adblock` trong Cargo.toml, phải xoá cache để build lại.
//! Kestrel và servo-gtk fork phải dùng cùng version.

use adblock::Engine;
use adblock::lists::ParseOptions;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/kestrel_filters.rs"));

static BLOB: OnceLock<Vec<u8>> = OnceLock::new();

/// Trả về adblock engine đã serialize. Cache trong process memory để
/// không đọc lại file cho mỗi tab.
pub fn get_blob() -> &'static [u8] {
    BLOB.get_or_init(build_or_load_blob)
}

fn cache_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("kestrel-browser")
}

fn blob_path() -> PathBuf {
    cache_dir().join("adblock.blob")
}

fn build_or_load_blob() -> Vec<u8> {
    let path = blob_path();

    if path.exists() {
        match fs::read(&path) {
            Ok(blob) if blob.len() > 4096 => {
                log::info!(
                    "Loaded adblock blob from cache ({} KB)",
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

    log::info!("Building adblock engine from rules (first run, 5-15s)...");
    let rules = rules_for_level(2); // Aggressive: EasyList + EasyPrivacy + Fanboy
    if rules.is_empty() {
        log::error!("No adblock rules embedded; adblock disabled");
        return Vec::new();
    }
    log::info!("Parsing {} rules...", rules.len());

    let engine = Engine::from_rules(&rules, ParseOptions::default());
    let blob = match engine.serialize_raw() {
        Ok(b) => b,
        Err(e) => {
            log::error!("Failed to serialize adblock engine: {e:?}");
            return Vec::new();
        }
    };

    log::info!("Built adblock blob ({} KB)", blob.len() / 1024);

    if let Err(e) = fs::create_dir_all(cache_dir()) {
        log::warn!("Failed to create adblock cache dir: {e}");
    } else if let Err(e) = fs::write(&path, &blob) {
        log::warn!("Failed to cache adblock blob at {path:?}: {e}");
    } else {
        log::info!("Cached adblock blob at {path:?}");
    }

    blob
}
