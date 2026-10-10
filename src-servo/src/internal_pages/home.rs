//! Trang chủ nội bộ — kestrel://home
//!
//! Render theo ngôn ngữ cấu hình. Mặc định tiếng Anh.

/// Render trang chủ theo mã ngôn ngữ (`en` hoặc `vi`).
pub fn render(lang: &str) -> String {
    match lang {
        "vi" => VI.to_string(),
        _ => EN.to_string(),
    }
}

const EN: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Kestrel — Home</title>
<style>
  :root { --bg:#fafafa; --fg:#1a1a1a; --accent:#0a84ff; --card:#fff; --border:#e5e5e5; --muted:#6b6b6b; }
  @media (prefers-color-scheme: dark) {
    :root { --bg:#1c1c1e; --fg:#f5f5f7; --card:#2c2c2e; --border:#3a3a3c; --muted:#8e8e93; }
  }
  * { box-sizing:border-box; margin:0; padding:0; }
  body { font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;
         background:var(--bg); color:var(--fg); min-height:100vh;
         display:flex; flex-direction:column; align-items:center;
         padding:80px 24px 24px; }
  .logo { font-size:48px; font-weight:700; letter-spacing:-1.5px; margin-bottom:8px; }
  .logo span { color:var(--accent); }
  .tagline { color:var(--muted); font-size:14px; margin-bottom:40px; }
  form { width:100%; max-width:560px; margin-bottom:48px; }
  input[type=text] { width:100%; padding:14px 20px; font-size:16px;
                     border:1px solid var(--border); border-radius:24px;
                     background:var(--card); color:var(--fg); outline:none; }
  input[type=text]:focus { border-color:var(--accent); }
  .cards { display:flex; flex-wrap:wrap; max-width:720px;
           justify-content:center; margin:-8px; }
  .card { flex:1 1 180px; max-width:220px; margin:8px; padding:20px;
          background:var(--card); border:1px solid var(--border);
          border-radius:12px; text-decoration:none; color:var(--fg); }
  .card:hover { border-color:var(--accent); }
  .card-title { font-size:15px; font-weight:600; margin-bottom:4px; }
  .card-desc { font-size:13px; color:var(--muted); line-height:1.4; }
  .footer { margin-top:auto; padding-top:48px; font-size:12px; color:var(--muted); }
</style>
</head>
<body>
  <div class="logo">Kestrel<span>.</span></div>
  <div class="tagline">Lightweight, pure-Rust, zero-telemetry browser</div>

  <form action="https://search.brave.com/search" method="get">
    <input type="text" name="q" placeholder="Search with Brave or enter URL..." autofocus>
  </form>

  <div class="cards">
    <a class="card" href="kestrel://history">
      <div class="card-title">History</div>
      <div class="card-desc">View pages you have visited</div>
    </a>
    <a class="card" href="kestrel://settings">
      <div class="card-title">Settings</div>
      <div class="card-desc">Homepage, search engine, language</div>
    </a>
    <a class="card" href="https://servo.org">
      <div class="card-title">Servo</div>
      <div class="card-desc">Learn about the engine behind Kestrel</div>
    </a>
  </div>

  <div class="footer">Kestrel Browser — Phase 1</div>
</body>
</html>"#;

const VI: &str = r#"<!DOCTYPE html>
<html lang="vi">
<head>
<meta charset="utf-8">
<title>Kestrel — Trang chủ</title>
<style>
  :root { --bg:#fafafa; --fg:#1a1a1a; --accent:#0a84ff; --card:#fff; --border:#e5e5e5; --muted:#6b6b6b; }
  @media (prefers-color-scheme: dark) {
    :root { --bg:#1c1c1e; --fg:#f5f5f7; --card:#2c2c2e; --border:#3a3a3c; --muted:#8e8e93; }
  }
  * { box-sizing:border-box; margin:0; padding:0; }
  body { font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;
         background:var(--bg); color:var(--fg); min-height:100vh;
         display:flex; flex-direction:column; align-items:center;
         padding:80px 24px 24px; }
  .logo { font-size:48px; font-weight:700; letter-spacing:-1.5px; margin-bottom:8px; }
  .logo span { color:var(--accent); }
  .tagline { color:var(--muted); font-size:14px; margin-bottom:40px; }
  form { width:100%; max-width:560px; margin-bottom:48px; }
  input[type=text] { width:100%; padding:14px 20px; font-size:16px;
                     border:1px solid var(--border); border-radius:24px;
                     background:var(--card); color:var(--fg); outline:none; }
  input[type=text]:focus { border-color:var(--accent); }
  .cards { display:flex; flex-wrap:wrap; max-width:720px;
           justify-content:center; margin:-8px; }
  .card { flex:1 1 180px; max-width:220px; margin:8px; padding:20px;
          background:var(--card); border:1px solid var(--border);
          border-radius:12px; text-decoration:none; color:var(--fg); }
  .card:hover { border-color:var(--accent); }
  .card-title { font-size:15px; font-weight:600; margin-bottom:4px; }
  .card-desc { font-size:13px; color:var(--muted); line-height:1.4; }
  .footer { margin-top:auto; padding-top:48px; font-size:12px; color:var(--muted); }
</style>
</head>
<body>
  <div class="logo">Kestrel<span>.</span></div>
  <div class="tagline">Trình duyệt nhẹ, thuần Rust, không telemetry</div>

  <form action="https://search.brave.com/search" method="get">
    <input type="text" name="q" placeholder="Tìm kiếm với Brave hoặc nhập URL..." autofocus>
  </form>

  <div class="cards">
    <a class="card" href="kestrel://history">
      <div class="card-title">Lịch sử</div>
      <div class="card-desc">Xem các trang đã truy cập</div>
    </a>
    <a class="card" href="kestrel://settings">
      <div class="card-title">Cài đặt</div>
      <div class="card-desc">Trang chủ, công cụ tìm kiếm, ngôn ngữ</div>
    </a>
    <a class="card" href="https://servo.org">
      <div class="card-title">Servo</div>
      <div class="card-desc">Tìm hiểu về engine đằng sau Kestrel</div>
    </a>
  </div>

  <div class="footer">Kestrel Browser — Phase 1</div>
</body>
</html>"#;
