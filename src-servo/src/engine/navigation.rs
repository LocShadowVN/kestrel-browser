//! Chuẩn hóa input omnibox thành URL, hỗ trợ công cụ tìm kiếm tùy chỉnh.

const DEFAULT_SEARCH_ENGINE: &str = "https://search.brave.com/search?q=";

pub fn normalize_url(input: &str) -> String {
    normalize_url_with_search(input, DEFAULT_SEARCH_ENGINE)
}

pub fn normalize_url_with_search(input: &str, search_engine: &str) -> String {
    let input = input.trim();

    if input.is_empty() {
        return "about:blank".into();
    }

    // Kestrel internal pages — giữ nguyên, không qua search.
    if input.starts_with("kestrel://") || input == "kestrel:" {
        return input.to_string();
    }

    if input.starts_with("http://")
        || input.starts_with("https://")
        || input.starts_with("about:")
        || input.starts_with("file://")
        || input.starts_with("data:")
    {
        return input.to_string();
    }

    if input.starts_with("localhost") || input.starts_with("127.0.0.1") || input.starts_with("[::1]")
    {
        return format!("http://{}", input);
    }

    if looks_like_domain_or_url(input) {
        return format!("https://{}", input);
    }

    let engine = if search_engine.trim().is_empty() {
        DEFAULT_SEARCH_ENGINE
    } else {
        search_engine.trim()
    };

    format!("{}{}", engine, urlencoding(input))
}

fn looks_like_domain_or_url(input: &str) -> bool {
    if input.contains(char::is_whitespace) || !input.contains('.') {
        return false;
    }

    // Tách phần host trước `/` hoặc `?` hoặc `:`
    let host_port = input.split(&['/', '?', '#'][..]).next().unwrap_or(input);
    let host = host_port.split(':').next().unwrap_or(host_port);

    if host.is_empty() || host.starts_with('.') || host.ends_with('.') {
        return false;
    }

    // Kiểm tra IPv4 (ví dụ 192.168.1.1:8080)
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() == 4 && parts.iter().all(|p| p.parse::<u8>().is_ok()) {
        return true;
    }

    // Kiểm tra TLD hợp lệ (>= 2 ký tự chữ cái hoặc punycode xn--)
    if let Some(tld) = parts.last() {
        let valid_tld = (tld.len() >= 2 && tld.chars().all(|c| c.is_ascii_alphabetic()))
            || tld.starts_with("xn--");
        let valid_labels = parts.iter().all(|label| {
            !label.is_empty()
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-')
        });
        return valid_tld && valid_labels;
    }

    false
}

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(normalize_url(""), "about:blank");
        assert_eq!(normalize_url("   "), "about:blank");
    }

    #[test]
    fn test_full_url() {
        assert_eq!(normalize_url("https://example.com"), "https://example.com");
        assert_eq!(normalize_url("http://example.com"), "http://example.com");
    }

    #[test]
    fn test_domain() {
        assert_eq!(normalize_url("example.com"), "https://example.com");
        assert_eq!(
            normalize_url("github.com/LocShadowVN/kestrel-browser"),
            "https://github.com/LocShadowVN/kestrel-browser"
        );
    }

    #[test]
    fn test_search() {
        assert!(normalize_url("hello world").starts_with("https://search.brave.com/search?q="));
    }

    #[test]
    fn test_custom_search_engine() {
        let url = normalize_url_with_search("rust servo", "https://duckduckgo.com/?q=");
        assert_eq!(url, "https://duckduckgo.com/?q=rust+servo");
    }

    #[test]
    fn test_kestrel_internal_scheme() {
        assert_eq!(normalize_url("kestrel://home"), "kestrel://home");
        assert_eq!(normalize_url("kestrel://settings"), "kestrel://settings");
        assert_eq!(normalize_url("kestrel://history"), "kestrel://history");
        assert_eq!(normalize_url("kestrel:"), "kestrel:");
    }

    #[test]
    fn test_kestrel_scheme_with_whitespace() {
        assert_eq!(normalize_url("  kestrel://home  "), "kestrel://home");
    }

    #[test]
    fn test_localhost() {
        assert_eq!(normalize_url("localhost:8080"), "http://localhost:8080");
        assert_eq!(normalize_url("127.0.0.1:3000"), "http://127.0.0.1:3000");
    }

    #[test]
    fn test_special_schemes() {
        assert_eq!(normalize_url("about:blank"), "about:blank");
        assert_eq!(
            normalize_url("file:///tmp/test.html"),
            "file:///tmp/test.html"
        );
        assert_eq!(
            normalize_url("data:text/plain,hello"),
            "data:text/plain,hello"
        );
    }
}
