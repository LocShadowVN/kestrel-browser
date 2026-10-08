use servo::WebViewDelegate;

/// Kestrel delegate — nhận sự kiện từ Servo.
///
/// Phase 1: chỉ log sự kiện. Chưa có adblock, chưa có network hook.
pub struct KestrelWebViewDelegate;

impl WebViewDelegate for KestrelWebViewDelegate {
    fn notify_page_title_changed(
        &self,
        _webview: servo::WebView,
        title: Option<String>,
    ) {
        log::info!("Title: {:?}", title);
    }

    fn notify_url_changed(
        &self,
        _webview: servo::WebView,
        url: url::Url,
    ) {
        log::info!("URL: {}", url);
    }
}
