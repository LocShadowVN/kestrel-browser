//! Trang chủ nội bộ — kestrel://home
//!
//! Song ngữ Anh-Việt, tiếng Anh là ngôn ngữ chính. Search bar dùng form
//! submit thay vì JS để không phụ thuộc vào message channel.

pub const HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Kestrel — Home</title>
<style>
  :root {
    --bg: #fafafa;
    --fg: #1a1a1a;
    --accent: #0a84ff;
    --card-bg: #ffffff;
    --card-border: #e5e5e5;
    --muted: #6b6b6b;
  }
  @media (prefers-color-scheme: dark) {
    :root {
      --bg: #1c1c1e;
      --fg: #f5f5f7;
      --accent: #0a84ff;
      --card-bg: #2c2c2e;
      --card-border: #3a3a3c;
      --muted: #8e8e93;
    }
  }
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    background: var(--bg);
    color: var(--fg);
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 72px 24px 24px;
  }
  .logo {
    font-size: 48px;
    font-weight: 700;
    letter-spacing: -1.5px;
    margin-bottom: 8px;
  }
  .logo span { color: var(--accent); }
  .tagline {
    color: var(--muted);
    font-size: 14px;
    margin-bottom: 40px;
  }
  .search {
    width: 100%;
    max-width: 560px;
    margin-bottom: 48px;
  }
  .search input {
    width: 100%;
    padding: 14px 20px;
    font-size: 16px;
    border: 1px solid var(--card-border);
    border-radius: 24px;
    background: var(--card-bg);
    color: var(--fg);
    outline: none;
    transition: border-color 0.15s;
  }
  .search input:focus { border-color: var(--accent); }

  .cards {
    display: flex;
    flex-wrap: wrap;
    width: 100%;
    max-width: 720px;
    justify-content: center;
    margin: -8px;
  }
  .card {
    flex: 1 1 180px;
    min-width: 180px;
    max-width: 220px;
    margin: 8px;
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 12px;
    padding: 20px;
    text-decoration: none;
    color: var(--fg);
    transition: transform 0.12s, border-color 0.12s;
    overflow-wrap: break-word;
    word-wrap: break-word;
  }
  .card:hover {
    transform: translateY(-2px);
    border-color: var(--accent);
  }
  .card-title {
    font-size: 15px;
    font-weight: 600;
    margin-bottom: 4px;
  }
  .card-desc {
    font-size: 13px;
    color: var(--muted);
    line-height: 1.4;
  }

  .lang-divider {
    width: 100%;
    max-width: 720px;
    margin: 56px 0 32px;
    border: none;
    border-top: 1px solid var(--card-border);
  }
  .section-title {
    font-size: 13px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 1.5px;
    color: var(--muted);
    margin-bottom: 8px;
    align-self: flex-start;
    max-width: 720px;
    width: 100%;
    padding-left: 8px;
  }
  .section-tagline {
    color: var(--muted);
    font-size: 13px;
    margin-bottom: 24px;
  }

  .footer {
    margin-top: auto;
    padding-top: 64px;
    font-size: 12px;
    color: var(--muted);
  }
</style>
</head>
<body>
  <!-- ===================== EN ===================== -->
  <div class="logo">Kestrel<span>.</span></div>
  <div class="tagline">Lightweight, pure-Rust, zero-telemetry browser</div>

  <form class="search" action="https://search.brave.com/search" method="get">
    <input type="text" name="q" placeholder="Search with Brave or enter URL..." autofocus>
  </form>

  <div class="cards">
    <a class="card" href="kestrel://history">
      <div class="card-title">History</div>
      <div class="card-desc">View pages you have visited</div>
    </a>
    <a class="card" href="kestrel://settings">
      <div class="card-title">Settings</div>
      <div class="card-desc">Homepage, search engine, appearance</div>
    </a>
    <a class="card" href="https://servo.org">
      <div class="card-title">Servo</div>
      <div class="card-desc">Learn about the engine behind Kestrel</div>
    </a>
  </div>

  <!-- ===================== VI ===================== -->
  <hr class="lang-divider">

  <div class="section-title">Tiếng Việt</div>
  <div class="section-tagline">Trình duyệt nhẹ, thuần Rust, không telemetry</div>

  <div class="cards">
    <a class="card" href="kestrel://history">
      <div class="card-title">Lịch sử</div>
      <div class="card-desc">Xem các trang đã truy cập</div>
    </a>
    <a class="card" href="kestrel://settings">
      <div class="card-title">Cài đặt</div>
      <div class="card-desc">Trang chủ, công cụ tìm kiếm, giao diện</div>
    </a>
    <a class="card" href="https://servo.org">
      <div class="card-title">Servo</div>
      <div class="card-desc">Tìm hiểu về engine đằng sau Kestrel</div>
    </a>
  </div>

  <div class="footer">Kestrel Browser — Phase 1</div>
</body>
</html>"#;
