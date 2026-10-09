//! Trang chủ nội bộ — kestrel://home

pub const HTML: &str = r#"<!DOCTYPE html>
<html lang="vi">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Kestrel Home</title>
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
    padding: 80px 24px 24px;
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
    position: relative;
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
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 16px;
    width: 100%;
    max-width: 720px;
  }
  .card {
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 12px;
    padding: 20px;
    text-decoration: none;
    color: var(--fg);
    transition: transform 0.12s, border-color 0.12s;
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
  .footer {
    margin-top: auto;
    padding-top: 48px;
    font-size: 12px;
    color: var(--muted);
  }
</style>
</head>
<body>
  <div class="logo">Kestrel<span>.</span></div>
  <div class="tagline">Trình duyệt nhẹ, thuần Rust, không telemetry</div>

  <div class="search">
    <input type="text" id="q" placeholder="Tìm kiếm hoặc nhập URL..." autofocus>
  </div>

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

  <script>
    document.getElementById('q').addEventListener('keydown', function (e) {
      if (e.key === 'Enter') {
        var v = this.value.trim();
        if (v) {
          // Gửi tín hiệu về native qua kênh message của servo-gtk.
          if (window.servoGtk && window.servoGtk.messageHandlers &&
              window.servoGtk.messageHandlers.kestrel) {
            window.servoGtk.messageHandlers.kestrel.postMessage(
              JSON.stringify({ action: 'navigate', url: v })
            );
          }
        }
      }
    });
  </script>
</body>
</html>"#;
