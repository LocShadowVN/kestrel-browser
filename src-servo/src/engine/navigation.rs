//! Chuẩn hóa input omnibox thành URL.

pub fn normalize_url(input: &str) -> String {
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

    if input.starts_with("localhost") || input.starts_with("127.0.0.1") {
        return format!("http://{}", input);
    }

    if input.contains('.')
        && !input.contains(' ')
        && input
            .split('.')
            .last()
            .map(|tld| tld.len() >= 2 && tld.chars().all(|c| c.is_ascii_alphabetic()))
            .unwrap_or(false)
    {
        return format!("https://{}", input);
    }

    format!(
        "https://search.brave.com/search?q={}",
        urlencoding(input)
    )
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
        assert_eq!(
            normalize_url("https://example.com"),
            "https://example.com"
        );
        assert_eq!(
            normalize_url("http://example.com"),
            "http://example.com"
        );
    }

    #[test]
    fn test_domain() {
        assert_eq!(
            normalize_url("example.com"),
            "https://example.com"
        );
    }

    #[test]
    fn test_search() {
        assert!(normalize_url("hello world").starts_with("https://search.brave.com/search?q="));
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
        assert_eq!(normalize_url("file:///tmp/test.html"), "file:///tmp/test.html");
        assert_eq!(normalize_url("data:text/plain,hello"), "data:text/plain,hello");
    }
}
