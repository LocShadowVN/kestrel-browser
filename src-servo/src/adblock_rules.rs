//! Filter rules, embed vào binary tại build time.
//!
//! Danh sách (EasyList, EasyPrivacy, Fanboy Annoyances) được tải bởi `build.rs`
//! và nhúng qua `include_str!`. Không tải runtime.

include!(concat!(env!("OUT_DIR"), "/kestrel_filters.rs"));

/// Rules cho level cho trước. Level 0 (Off) trả về rỗng.
pub fn for_level(level: u8) -> Vec<String> {
    rules_for_level(level)
}
