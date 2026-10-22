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

    let lower = input.to_ascii_lowercase();

    // Kestrel internal pages — giữ nguyên, chuẩn hóa scheme về chữ thường.
    if lower.starts_with("kestrel://") || lower == "kestrel:" {
        let rest = &input[input.find(':').map(|i| i + 1).unwrap_or(input.len())..];
        return format!("kestrel:{}", rest);
    }

    if lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("about:")
        || lower.starts_with("file://")
        || lower.starts_with("data:")
    {
        if let Some(colon_idx) = input.find(':') {
            return format!(
                "{}{}",
                &lower[..colon_idx],
                &input[colon_idx..]
            );
        }
        return input.to_string();
    }

    if is_local_address(input) {
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

fn is_local_address(input: &str) -> bool {
    if input.contains(char::is_whitespace) {
        return false;
    }

    let host_port = input.split(&['/', '?', '#'][..]).next().unwrap_or(input);
    let host_port_lower = host_port.to_ascii_lowercase();

    // IPv6 loopback [::1] hoặc [::1]:port
    if let Some(rest) = host_port_lower.strip_prefix("[::1]") {
        return rest.is_empty() || rest.strip_prefix(':').map_or(false, is_valid_port);
    }

    let (host, port_opt) = match host_port_lower.split_once(':') {
        Some((h, p)) => (h, Some(p)),
        None => (host_port_lower.as_str(), None),
    };

    if let Some(port) = port_opt {
        if !is_valid_port(port) {
            return false;
        }
    }

    host == "localhost"
        || host.ends_with(".localhost")
        || host == "127.0.0.1"
        || host == "0.0.0.0"
}

fn is_valid_port(port: &str) -> bool {
    !port.is_empty() && port.parse::<u16>().map_or(false, |p| p > 0)
}

fn looks_like_domain_or_url(input: &str) -> bool {
    if input.contains(char::is_whitespace) || !input.contains('.') {
        return false;
    }

    // Tách phần host[:port] trước `/`, `?`, `#`
    let host_port = input.split(&['/', '?', '#'][..]).next().unwrap_or(input);
    if host_port.is_empty() {
        return false;
    }

    let (host, port_opt) = match host_port.split_once(':') {
        Some((h, p)) => (h, Some(p)),
        None => (host_port, None),
    };

    if let Some(port) = port_opt {
        if !is_valid_port(port) {
            return false;
        }
    }

    if host.is_empty() || host.starts_with('.') || host.ends_with('.') {
        return false;
    }

    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() < 2 {
        return false;
    }

    // Kiểm tra IPv4 (ví dụ 192.168.1.1:8080)
    if parts.len() == 4 && parts.iter().all(|p| p.parse::<u8>().is_ok()) {
        return true;
    }

    // Kiểm tra TLD hợp lệ (>= 2 ký tự chữ cái hoặc punycode xn--)
    if let Some(tld) = parts.last() {
        let tld_lower = tld.to_ascii_lowercase();
        let valid_tld = (tld_lower.len() >= 2
            && tld_lower.chars().all(|c| c.is_ascii_alphabetic()))
            || (tld_lower.starts_with("xn--") && tld_lower.len() > 4);
        let valid_labels = parts.iter().all(|label| {
            !label.is_empty()
                && label.len() <= 63
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
        assert_eq!(normalize_url("HTTPS://Example.com/Path"), "https://Example.com/Path");
    }

    #[test]
    fn test_domain() {
        assert_eq!(normalize_url("example.com"), "https://example.com");
        assert_eq!(
            normalize_url("github.com/LocShadowVN/kestrel-browser"),
            "https://github.com/LocShadowVN/kestrel-browser"
        );
        assert_eq!(
            normalize_url("192.168.1.1:8080/admin"),
            "https://192.168.1.1:8080/admin"
        );
    }

    #[test]
    fn test_search_and_non_domains() {
        assert!(normalize_url("hello world").starts_with("https://search.brave.com/search?q="));
        assert!(
            normalize_url("localhost vs 127.0.0.1")
                .starts_with("https://search.brave.com/search?q=")
        );
        assert!(
            normalize_url("what/is.this")
                .starts_with("https://search.brave.com/search?q=")
        );
        assert!(
            normalize_url("foo:bar.com")
                .starts_with("https://search.brave.com/search?q=")
        );
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
        assert_eq!(normalize_url("KESTREL://about"), "kestrel://about");
        assert_eq!(normalize_url("kestrel:"), "kestrel:");
    }

    #[test]
    fn test_kestrel_scheme_with_whitespace() {
        assert_eq!(normalize_url("  kestrel://home  "), "kestrel://home");
    }

    #[test]
    fn test_localhost() {
        assert_eq!(normalize_url("localhost"), "http://localhost");
        assert_eq!(normalize_url("localhost:8080"), "http://localhost:8080");
        assert_eq!(normalize_url("127.0.0.1:3000/api"), "http://127.0.0.1:3000/api");
        assert_eq!(normalize_url("[::1]:5173"), "http://[::1]:5173");
        assert_eq!(normalize_url("localhost.com"), "https://localhost.com");
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
