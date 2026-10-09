//! Trang cài đặt nội bộ — kestrel://settings

pub const HTML: &str = r#"<!DOCTYPE html>
<html lang="vi">
<head>
<meta charset="utf-8">
<title>Kestrel Settings</title>
<style>
  :root { --bg: #fafafa; --fg: #1a1a1a; --card-bg: #fff; --card-border: #e5e5e5; --muted: #6b6b6b; --accent: #0a84ff; }
  @media (prefers-color-scheme: dark) {
    :root { --bg: #1c1c1e; --fg: #f5f5f7; --card-bg: #2c2c2e; --card-border: #3a3a3c; --muted: #8e8e93; }
  }
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: var(--bg); color: var(--fg); padding: 48px 24px; }
  .container { max-width: 640px; margin: 0 auto; }
  h1 { font-size: 28px; font-weight: 700; margin-bottom: 32px; letter-spacing: -0.5px; }
  h2 { font-size: 13px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; color: var(--muted); margin: 32px 0 12px; }
  .field { background: var(--card-bg); border: 1px solid var(--card-border); border-radius: 12px; padding: 16px 20px; margin-bottom: 10px; }
  .field label { display: block; font-size: 14px; font-weight: 600; margin-bottom: 6px; }
  .field .hint { font-size: 12px; color: var(--muted); margin-bottom: 10px; line-height: 1.4; }
  .field input[type="text"] { width: 100%; padding: 8px 12px; font-size: 14px; border: 1px solid var(--card-border); border-radius: 8px; background: var(--bg); color: var(--fg); outline: none; }
  .field input[type="text"]:focus { border-color: var(--accent); }
  .toggle { display: flex; align-items: center; justify-content: space-between; }
  .toggle-switch { position: relative; width: 48px; height: 28px; background: var(--card-border); border-radius: 14px; cursor: pointer; transition: background 0.2s; }
  .toggle-switch.on { background: var(--accent); }
  .toggle-switch::after { content: ""; position: absolute; top: 3px; left: 3px; width: 22px; height: 22px; background: #fff; border-radius: 50%; transition: transform 0.2s; }
  .toggle-switch.on::after { transform: translateX(20px); }
  .actions { margin-top: 32px; display: flex; gap: 12px; }
  button { padding: 10px 20px; font-size: 14px; font-weight: 600; border: none; border-radius: 8px; cursor: pointer; }
  .btn-primary { background: var(--accent); color: #fff; }
  .btn-secondary { background: var(--card-bg); color: var(--fg); border: 1px solid var(--card-border); }
  .status { margin-top: 16px; font-size: 13px; color: var(--muted); }
</style>
</head>
<body>
  <div class="container">
    <h1>Cài đặt</h1>

    <h2>Chung</h2>
    <div class="field">
      <label for="homepage">Trang chủ</label>
      <div class="hint">Trang sẽ mở khi bạn nhấn nút Home hoặc khởi động trình duyệt.</div>
      <input type="text" id="homepage" value="kestrel://home">
    </div>
    <div class="field">
      <label for="search">Công cụ tìm kiếm</label>
      <div class="hint">URL template, dùng {q} cho từ khóa tìm kiếm.</div>
      <input type="text" id="search" value="https://search.brave.com/search?q=">
    </div>

    <h2>Giao diện</h2>
    <div class="field">
      <div class="toggle">
        <div>
          <label>Chế độ tối</label>
          <div class="hint">Ưu tiên giao diện tối cho toàn hệ thống.</div>
        </div>
        <div class="toggle-switch" id="dark-toggle"></div>
      </div>
    </div>

    <div class="actions">
      <button class="btn-primary" id="save">Lưu</button>
      <button class="btn-secondary" id="reset">Khôi phục mặc định</button>
    </div>
    <div class="status" id="status"></div>
  </div>

  <script>
    var darkToggle = document.getElementById('dark-toggle');
    darkToggle.addEventListener('click', function () {
      this.classList.toggle('on');
    });

    document.getElementById('save').addEventListener('click', function () {
      var payload = {
        action: 'save-settings',
        homepage: document.getElementById('homepage').value,
        search_engine: document.getElementById('search').value,
        dark_theme: darkToggle.classList.contains('on')
      };
      if (window.servoGtk && window.servoGtk.messageHandlers && window.servoGtk.messageHandlers.kestrel) {
        window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify(payload));
        document.getElementById('status').textContent = 'Đã lưu.';
      } else {
        document.getElementById('status').textContent = 'Không thể lưu — kênh native chưa sẵn sàng.';
      }
    });

    document.getElementById('reset').addEventListener('click', function () {
      document.getElementById('homepage').value = 'kestrel://home';
      document.getElementById('search').value = 'https://search.brave.com/search?q=';
      darkToggle.classList.remove('on');
      document.getElementById('status').textContent = 'Đã khôi phục mặc định (chưa lưu).';
    });
  </script>
</body>
</html>"#;
