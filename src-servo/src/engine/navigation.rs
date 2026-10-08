/// Chuẩn hóa input từ omnibox thành URL.
pub fn normalize_url(input: &str) -> String {
    let input = input.trim();

    if input.is_empty() {
        return "about:blank".into();
    }

    if input.starts_with("http://")
        || input.starts_with("https://")
        || input.starts_with("about:")
        || input.starts_with("file://")
    {
        return input.to_string();
    }

    if input.starts_with("localhost") || input.starts_with("127.0.0.1") {
        return format!("http://{}", input);
    }

    // Kiểm tra có phải domain
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

    // Search
    format!(
        "https://search.brave.com/search?q={}",
        urlencoding(input)
    )
}

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
