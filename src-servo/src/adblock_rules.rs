// src-servo/src/adblock_rules.rs
//! Adblock engine blob — chỉ biên dịch các quy tắc chặn tầng mạng (Network rules),
//! hỗ trợ phân tách theo cấp độ Standard (Level 1) và Aggressive (Level 2),
//! nạp cực nhanh từ đĩa không gây khựng luồng giao diện GTK4.

use adblock::lists::ParseOptions;
use adblock::Engine;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/kestrel_filters.rs"));

static BLOB_STANDARD: OnceLock<Vec<u8>> = OnceLock::new();
static BLOB_AGGRESSIVE: OnceLock<Vec<u8>> = OnceLock::new();
static ACTIVE_LEVEL: AtomicU8 = AtomicU8::new(2);

const MIN_BLOB_BYTES: usize = 4096;

/// Cập nhật mức độ chặn mặc định cho các lần gọi `get_blob()` / `try_get_blob()`.
pub fn set_active_level(level: u8) {
    ACTIVE_LEVEL.store(level.min(2), Ordering::Relaxed);
}

/// Trả về adblock engine đã serialize theo level đang kích hoạt (chờ nếu đang build).
pub fn get_blob() -> &'static [u8] {
    let level = ACTIVE_LEVEL.load(Ordering::Relaxed);
    if level == 0 {
        return &[];
    }
    get_blob_for_level(level)
}

/// Trả về adblock engine đã serialize cho một level cụ thể (1 = Standard, 2 = Aggressive).
pub fn get_blob_for_level(level: u8) -> &'static [u8] {
    match level {
        0 => &[],
        1 => BLOB_STANDARD
            .get_or_init(|| build_or_load_blob_for_level(1))
            .as_slice(),
        _ => BLOB_AGGRESSIVE
            .get_or_init(|| build_or_load_blob_for_level(2))
            .as_slice(),
    }
}

/// Lấy adblock blob không khóa luồng UI nếu đang biên dịch lần đầu;
/// nếu đã có sẵn file cache trên đĩa thì nạp tức thì (< 1ms).
pub fn try_get_blob() -> Option<&'static [u8]> {
    let level = ACTIVE_LEVEL.load(Ordering::Relaxed);
    try_get_blob_for_level(level)
}

/// Lấy adblock blob theo level cụ thể mà không làm đơ luồng giao diện GTK4.
pub fn try_get_blob_for_level(level: u8) -> Option<&'static [u8]> {
    match level {
        0 => None,
        1 => {
            if let Some(b) = BLOB_STANDARD.get() {
                return (!b.is_empty()).then_some(b.as_slice());
            }
            if blob_path_for_level(1).exists() {
                let b = get_blob_for_level(1);
                return (!b.is_empty()).then_some(b);
            }
            if let Some(b) = BLOB_AGGRESSIVE.get() {
                return (!b.is_empty()).then_some(b.as_slice());
            }
            None
        }
        _ => {
            if let Some(b) = BLOB_AGGRESSIVE.get() {
                return (!b.is_empty()).then_some(b.as_slice());
            }
            if blob_path_for_level(2).exists() {
                let b = get_blob_for_level(2);
                return (!b.is_empty()).then_some(b);
            }
            None
        }
    }
}

fn cache_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("kestrel-browser")
}

fn blob_path_for_level(level: u8) -> PathBuf {
    match level {
        1 => cache_dir().join("adblock_std_v3.blob"),
        _ => cache_dir().join("adblock_net_v3.blob"),
    }
}

fn is_cosmetic_or_comment(line: &str) -> bool {
    let l = line.trim();
    if l.is_empty()
        || l.starts_with('!')
        || l.starts_with('[')
        || l.starts_with('#')
    {
        return true;
    }

    l.contains("##")
        || l.contains("#@#")
        || l.contains("#?#")
        || l.contains("#@?#")
        || l.contains("#$#")
        || l.contains("#@$#")
        || l.contains("#%#")
        || l.contains("#@%#")
        || l.contains("$$")
        || l.contains("$@$")
        || l.contains("+js(")
}

fn filter_network_only_rules(raw_rules: Vec<String>) -> Vec<String> {
    raw_rules
        .into_iter()
        .filter(|line| !is_cosmetic_or_comment(line))
        .collect()
}

fn build_or_load_blob_for_level(level: u8) -> Vec<u8> {
    let normalized_level = if level == 1 { 1 } else { 2 };
    let path = blob_path_for_level(normalized_level);

    if let Some(cached) = try_read_cached_blob(&path, normalized_level) {
        return cached;
    }

    log::info!(
        "Building optimized network adblock engine (level {})...",
        normalized_level
    );
    let raw_rules = rules_for_level(normalized_level);
    if raw_rules.is_empty() {
        log::error!("No adblock rules embedded; adblock disabled");
        return Vec::new();
    }

    let rules = filter_network_only_rules(raw_rules);
    if rules.is_empty() {
        log::error!("No valid network rules after filtering; adblock disabled");
        return Vec::new();
    }
    log::info!(
        "Compiling {} network filter rules for level {}...",
        rules.len(),
        normalized_level
    );

    let engine = Engine::from_rules(&rules, ParseOptions::default());
    let blob = match engine.serialize_raw() {
        Ok(b) => b,
        Err(e) => {
            log::error!("Failed to serialize adblock engine: {e:?}");
            return Vec::new();
        }
    };

    log::info!(
        "Built optimized adblock blob for level {} ({} KB)",
        normalized_level,
        blob.len() / 1024
    );

    write_blob_atomically(&path, &blob, normalized_level);
    blob
}

fn try_read_cached_blob(path: &Path, level: u8) -> Option<Vec<u8>> {
    if !path.exists() {
        return None;
    }
    match fs::read(path) {
        Ok(blob) if blob.len() > MIN_BLOB_BYTES => {
            log::info!(
                "Loaded network adblock blob (level {}) from cache ({} KB)",
                level,
                blob.len() / 1024
            );
            Some(blob)
        }
        Ok(blob) => {
            log::warn!(
                "Cached adblock blob at {path:?} too small ({} bytes); rebuilding",
                blob.len()
            );
            let _ = fs::remove_file(path);
            None
        }
        Err(e) => {
            log::warn!("Failed to read cached adblock blob at {path:?}: {e}");
            None
        }
    }
}

fn write_blob_atomically(path: &Path, blob: &[u8], level: u8) {
    let dir = cache_dir();
    if let Err(e) = fs::create_dir_all(&dir) {
        log::warn!("Failed to create adblock cache dir: {e}");
        return;
    }

    let tmp_path = dir.join(format!(
        "adblock_lvl{}_{}.blob.tmp",
        level,
        std::process::id()
    ));
    if let Err(e) = fs::write(&tmp_path, blob) {
        log::warn!("Failed to write temporary adblock blob at {tmp_path:?}: {e}");
        return;
    }

    if let Err(e) = fs::rename(&tmp_path, path) {
        log::warn!("Failed to finalize adblock blob cache at {path:?}: {e}");
        let _ = fs::remove_file(&tmp_path);
    } else {
        log::info!("Cached adblock blob (level {}) atomically at {path:?}", level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosmetic_and_comment_filter() {
        let raw = vec![
            "! Comment".to_string(),
            "[Adblock Plus 2.0]".to_string(),
            "||doubleclick.net^$third-party".to_string(),
            "example.com##.ads-banner".to_string(),
            "example.com#@#.ads-banner".to_string(),
            "example.com##+js(nobab)".to_string(),
            "||googlesyndication.com/pagead/js/adsbygoogle.js".to_string(),
        ];
        let filtered = filter_network_only_rules(raw);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0], "||doubleclick.net^$third-party");
        assert_eq!(
            filtered[1],
            "||googlesyndication.com/pagead/js/adsbygoogle.js"
        );
    }
}
