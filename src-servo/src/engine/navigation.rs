// src-servo/src/engine/navigation.rs
//! Chuẩn hóa input omnibox thành URL, hỗ trợ IPv4/IPv6, localhost và công cụ tìm kiếm tùy chỉnh (`%s`).

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
            return format!("{}{}", &lower[..colon_idx], &input[colon_idx..]);
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

    let encoded_query = urlencoding(input);
    if engine.contains("%s") {
        engine.replace("%s", &encoded_query)
    } else {
        format!("{}{}", engine, encoded_query)
    }
}

/// Tách chuỗi `host_port` thành `(host, Option<port>)`, xử lý chính xác cả IPv6 dạng `[::1]:8080`.
fn split_host_and_port(host_port: &str) -> Option<(&str, Option<&str>)> {
    if host_port.starts_with('[') {
        let close_bracket = host_port.find(']')?;
        let ipv6_host = &host_port[..=close_bracket];
        let rest = &host_port[close_bracket + 1..];
        if rest.is_empty() {
            return Some((ipv6_host, None));
        }
        let port = rest.strip_prefix(':')?;
        if !is_valid_port(port) {
            return None;
        }
        return Some((ipv6_host, Some(port)));
    }

    match host_port.split_once(':') {
        Some((h, p)) => {
            if !is_valid_port(p) {
                return None;
            }
            Some((h, Some(p)))
        }
        None => Some((host_port, None)),
    }
}

fn is_local_address(input: &str) -> bool {
    if input.contains(char::is_whitespace) {
        return false;
    }

    let host_port = input.split(&['/', '?', '#'][..]).next().unwrap_or(input);
    let Some((host, _port)) = split_host_and_port(host_port) else {
        return false;
    };

    let host_lower = host.to_ascii_lowercase();
    if host_lower == "[::1]"
        || host_lower == "[0:0:0:0:0:0:0:1]"
        || host_lower == "localhost"
        || host_lower.ends_with(".localhost")
        || host_lower == "0.0.0.0"
    {
        return true;
    }

    // Kiểm tra dải loopback 127.0.0.0/8
    let octets: Vec<&str> = host_lower.split('.').collect();
    if octets.len() == 4
        && octets[0] == "127"
        && octets.iter().all(|o| o.parse::<u8>().is_ok())
    {
        return true;
    }

    false
}

fn is_valid_port(port: &str) -> bool {
    !port.is_empty() && port.parse::<u16>().map_or(false, |p| p > 0)
}

fn looks_like_domain_or_url(input: &str) -> bool {
    if input.contains(char::is_whitespace) {
        return false;
    }

    let host_port = input.split(&['/', '?', '#'][..]).next().unwrap_or(input);
    if host_port.is_empty() {
        return false;
    }

    let Some((host, _port)) = split_host_and_port(host_port) else {
        return false;
    };

    // Kiểm tra địa chỉ IPv6 dạng [2606:4700:4700::1111]
    if let Some(inner) = host.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        return !inner.is_empty()
            && inner.contains(':')
            && inner
                .chars()
                .all(|c| c.is_ascii_hexdigit() || c == ':' || c == '.');
    }

    if !host.contains('.') || host.starts_with('.') || host.ends_with('.') {
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
        assert_eq!(
            normalize_url("HTTPS://Example.com/Path"),
            "https://Example.com/Path"
        );
    }

    #[test]
    fn test_domain_and_ipv6() {
        assert_eq!(normalize_url("example.com"), "https://example.com");
        assert_eq!(
            normalize_url("github.com/LocShadowVN/kestrel-browser"),
            "https://github.com/LocShadowVN/kestrel-browser"
        );
        assert_eq!(
            normalize_url("192.168.1.1:8080/admin"),
            "https://192.168.1.1:8080/admin"
        );
        assert_eq!(
            normalize_url("[2606:4700:4700::1111]:443/dns-query"),
            "https://[2606:4700:4700::1111]:443/dns-query"
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
    fn test_custom_search_engine_and_placeholder() {
        let url = normalize_url_with_search("rust servo", "https://duckduckgo.com/?q=");
        assert_eq!(url, "https://duckduckgo.com/?q=rust+servo");

        let url_with_placeholder = normalize_url_with_search(
            "rust servo",
            "https://en.wikipedia.org/w/index.php?search=%s&title=Special:Search",
        );
        assert_eq!(
            url_with_placeholder,
            "https://en.wikipedia.org/w/index.php?search=rust+servo&title=Special:Search"
        );
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
        assert_eq!(
            normalize_url("127.0.0.1:3000/api"),
            "http://127.0.0.1:3000/api"
        );
        assert_eq!(
            normalize_url("127.0.1.1:5000"),
            "http://127.0.1.1:5000"
        );
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
