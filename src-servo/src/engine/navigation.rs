//! Chuẩn hóa input omnibox thành URL.

pub fn normalize_url(input: &str) -> String {
    let input = input.trim();

    if input.is_empty() {
        return "about:blank".into();
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
}
