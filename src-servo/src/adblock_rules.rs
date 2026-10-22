//! Adblock engine blob — chỉ biên dịch các quy tắc chặn tầng mạng (Network rules),
//! loại bỏ các quy tắc ẩn CSS (`##`, `#@#`, `#?#`) không dùng tới trong `load_web_resource`,
//! ghi đệm atomic và kiểm tra tính toàn vẹn khi nạp.

use adblock::lists::ParseOptions;
use adblock::Engine;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/kestrel_filters.rs"));

static BLOB: OnceLock<Vec<u8>> = OnceLock::new();

/// Trả về adblock engine đã serialize (đồng bộ an toàn qua OnceLock).
pub fn get_blob() -> &'static [u8] {
    BLOB.get_or_init(build_or_load_blob)
}

/// Lấy adblock blob cho tab. Đảm bảo kể cả tab đầu tiên lúc mới mở trình duyệt
/// cũng luôn nhận được dữ liệu bộ lọc thay vì bị `None` do race condition.
pub fn try_get_blob() -> Option<&'static [u8]> {
    let blob = get_blob();
    if blob.is_empty() {
        None
    } else {
        Some(blob)
    }
}

fn cache_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("kestrel-browser")
}

fn blob_path() -> PathBuf {
    cache_dir().join("adblock_net_v3.blob")
}

/// Lọc giữ lại đúng các rule tầng mạng (network filter rules), bỏ comment và cosmetic/scriptlet rules.
fn filter_network_only_rules(raw_rules: Vec<String>) -> Vec<String> {
    raw_rules
        .into_iter()
        .filter(|line| {
            let l = line.trim();
            !l.is_empty()
                && !l.starts_with('!')
                && !l.starts_with('[')
                && !l.starts_with('#')
                && !l.contains("##")
                && !l.contains("#@#")
                && !l.contains("#?#")
                && !l.contains("#@?#")
                && !l.contains("#$#")
                && !l.contains("#@$#")
                && !l.contains("#%#")
                && !l.contains("#@%#")
                && !l.contains("$$")
                && !l.contains("$@$")
                && !l.contains("+js(")
        })
        .collect()
}

fn is_valid_blob(blob: &[u8]) -> bool {
    if blob.len() <= 4096 {
        return false;
    }
    let mut engine = Engine::default();
    engine.deserialize(blob).is_ok()
}

fn build_or_load_blob() -> Vec<u8> {
    let path = blob_path();

    if path.exists() {
        match fs::read(&path) {
            Ok(blob) if is_valid_blob(&blob) => {
                log::info!(
                    "Loaded verified network adblock blob from cache ({} KB)",
                    blob.len() / 1024
                );
                return blob;
            }
            Ok(blob) => {
                log::warn!(
                    "Cached adblock blob invalid or corrupted ({} bytes); rebuilding",
                    blob.len()
                );
                let _ = fs::remove_file(&path);
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
    if rules.is_empty() {
        log::error!("No valid network rules after filtering; adblock disabled");
        return Vec::new();
    }
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

    let dir = cache_dir();
    if let Err(e) = fs::create_dir_all(&dir) {
        log::warn!("Failed to create adblock cache dir: {e}");
    } else {
        let tmp_path = dir.join("adblock_net_v3.blob.tmp");
        if let Err(e) = fs::write(&tmp_path, &blob) {
            log::warn!("Failed to write temporary adblock blob at {tmp_path:?}: {e}");
        } else if let Err(e) = fs::rename(&tmp_path, &path) {
            log::warn!("Failed to finalize adblock blob cache at {path:?}: {e}");
            let _ = fs::remove_file(&tmp_path);
        } else {
            log::info!("Cached adblock blob atomically at {path:?}");
        }
    }

    blob
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_network_only_rules() {
        let sample = vec![
            "! Comment line".to_string(),
            "[Adblock Plus 2.0]".to_string(),
            "||doubleclick.net^$third-party".to_string(),
            "example.com##.ad-banner".to_string(),
            "example.com#@#.ad-banner".to_string(),
            "example.com#?#.ad-slot:-abp-has(.sponsored)".to_string(),
            "example.com##+js(nobab)".to_string(),
            "||googlesyndication.com/pagead/".to_string(),
        ];
        let filtered = filter_network_only_rules(sample);
        assert_eq!(
            filtered,
            vec![
                "||doubleclick.net^$third-party".to_string(),
                "||googlesyndication.com/pagead/".to_string(),
            ]
        );
    }
}
