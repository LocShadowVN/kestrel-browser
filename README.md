<div align="center">

# Kestrel Browser

Trình duyệt desktop nhẹ cho Linux, xây trên **Servo engine** — viết bằng Rust.
Không WebKitGTK. Không Chromium. Không telemetry.

[![Status](https://img.shields.io/badge/status-alpha-orange.svg?style=flat-square)](#trạng-thái)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20x86__64-lightgrey.svg?style=flat-square)](#cài-đặt)
[![Engine](https://img.shields.io/badge/engine-Servo-orange.svg?style=flat-square)](https://servo.org/)
[![Rust](https://img.shields.io/badge/rust-2021-blue.svg?style=flat-square)](https://www.rust-lang.org/)

[Tiếng Việt](#tiếng-việt) · [English](#english)

</div>

---

> ⚠️ **Alpha — đang phát triển.**
>
> Đây là bản viết lại hoàn toàn, chuyển từ WebKitGTK + Tauri sang **Servo engine**.
> Hiện tại **chưa có adblock**, **chưa có vault**, **chưa hỗ trợ YouTube**.
> Đây là bản thử nghiệm để đánh giá Servo như một nền tảng dài hạn.
>
> Dùng hàng ngày: **chưa khuyến nghị**. Dùng để thử và báo lỗi: **rất hoan nghênh**.

---

<a name="tiếng-việt"></a>
## Tiếng Việt

### Tại sao lại viết lại?

Phiên bản trước (Vibird) dùng WebKitGTK qua Tauri. Nó chạy được, có adblock 4 tầng,
có vault, có download. Nhưng có một vấn đề không fix được:

**WebKitGTK bị bug compositor trên Nvidia + Wayland.** App có thể bị OOM kill,
văng ra login screen sau vài phút sử dụng. Đây là bug của WebKitGTK, không phải
của app — nó ảnh hưởng đến mọi ứng dụng dùng WebKitGTK (GNOME Web, Devhelp, Yelp).

Sau khi cân nhắc nhiều engine (WebKitGTK, CEF, Blitz, Sciter, Ultralight), chỉ có
**Servo** đáp ứng 3 tiêu chí:
- **Viết bằng Rust** (cùng hệ sinh thái, dễ đóng góp)
- **Nhẹ và có thiết kế để nhúng** (không bundle Chromium 200 MB)
- **Có tương lai** (Linux Foundation Europe bảo trợ, Igalia phát triển chính)

Kestrel là canh bạc dài hạn vào Servo. Chấp nhận đánh đổi: web compatibility
thấp hơn WebKit (78% WPT vs 96%), nhưng thoát khỏi bug Nvidia + Wayland.

### Mục tiêu

1. **Nhẹ thật.** RAM thấp, binary nhỏ, không bundle engine nặng.
2. **Không bug Nvidia.** Servo render qua WebRender + wgpu, không dùng GLX cũ.
3. **Thuần Rust.** Không C++, không FFI unsafe, dễ đọc, dễ đóng góp.
4. **Chạy được web cơ bản.** Đọc báo, tra cứu, vào GitHub, xem Wikipedia.

**Không phải mục tiêu**: thay Chrome, chạy YouTube 4K, cài extension Chrome.
Kestrel là **browser phụ** cho workflow đơn giản.

### Trạng thái

#### ✅ Đã có (Phase 1)

| Tính năng | Ghi chú |
|---|---|
| Servo engine | Render qua WebRender |
| GTK4 UI chrome | Native, không web-based |
| Multi-tab | GTK4 Notebook |
| Omnibox | URL + search + autocomplete |
| Back / forward / reload | Cơ bản |
| History | SQLite |
| Bookmarks | Add / remove |
| Settings dialog | Search engine, homepage |
| Session restore | Lưu tab khi thoát |
| Keyboard shortcuts | Ctrl+T, Ctrl+W, Ctrl+L, F5, ... |

#### ⚠️ Chưa có (Phase 2+)

| Tính năng | Ghi chú |
|---|---|
| Adblock | Chưa. Không có tầng nào. |
| Vault | Chưa. |
| Download manager | Chưa. |
| Extension support | Chưa. |
| Auto-update | Chưa. |
| Privacy shields | Chưa. |

#### ❌ Không hỗ trợ

| Tính năng | Lý do |
|---|---|
| YouTube video | Servo chưa có MSE |
| Netflix, Spotify Web | Thiếu DRM (Widevine) |
| Google Meet, Teams | Thiếu WebCodecs |
| Extension Chrome/Firefox | Không tương thích |

### Tương thích web (dự kiến)

Dựa trên Servo's WPT pass rate (~78%):

| Loại site | Tương thích | Ghi chú |
|---|---|---|
| Trang tĩnh (blog, news) | ✅ Tốt | VNExpress, BBC, Wikipedia |
| GitHub | ✅ Tốt | Render ổn |
| Documentation | ✅ Tốt | MDN, docs.rs |
| Google Search | ✅ Tốt | |
| Gmail | ⚠️ Một phần | Có thể lỗi layout |
| YouTube | ❌ Kém | Giao diện OK, không phát video |
| Facebook | ⚠️ Một phần | Feed load, widget lỗi |
| React/Vue SPA | ⚠️ Tùy site | Layout có thể vỡ |

**Đọc báo, tra cứu, GitHub** → ổn.
**YouTube, Netflix, Google Meet** → không.

### Kiến trúc

```
┌───────────────────────────────────────────────────────────┐
│                     GTK4 Application                       │
├───────────────────────────────────────────────────────────┤
│  Main Window                                               │
│    ├── Toolbar  (back/forward/reload + omnibox + menu)    │
│    ├── Tab Bar  (GTK4 Notebook)                           │
│    └── Content Area                                       │
│         └── Servo WebView (per tab)                       │
└───────────────────────────────────────────────────────────┘
                        │
                        ▼
┌───────────────────────────────────────────────────────────┐
│              Servo engine (Rust, native)                   │
│  WebRender · Stylo · SpiderMonkey · servo-media           │
└───────────────────────────────────────────────────────────┘
                        │
                        ▼
┌───────────────────────────────────────────────────────────┐
│                  Vibird Core (Rust)                        │
│  SQLite · Session · Config                                │
└───────────────────────────────────────────────────────────┘
```

Không có lớp Tauri. Không có webview lồng webview. GTK4 chrome gọi Servo API
trực tiếp qua Rust function calls.

### Cấu trúc thư mục

```
kestrel-browser/
├── Cargo.toml              # Workspace root
├── shared/                 # Common types
├── src-servo/              # Main app
│   ├── src/
│   │   ├── main.rs         # Entry
│   │   ├── app.rs          # GTK4 Application
│   │   ├── chrome/         # UI shell (window, toolbar, omnibox, tabbar)
│   │   ├── engine/         # Servo wrapper (browser, tab, delegate)
│   │   ├── views/          # Settings, history, bookmarks
│   │   ├── storage/        # SQLite, session
│   │   └── util/           # Config, log
│   └── resources/
│       ├── kestrel.css     # GTK4 styling
│       └── newtab.html     # New tab page
├── flatpak/                # Flatpak packaging
└── .github/workflows/      # CI
```

### Cài đặt

**Yêu cầu:**
- Linux x86_64
- Vulkan hoặc OpenGL 3.3+
- GTK4 (thường có sẵn trên Ubuntu 22.04+, Fedora 37+)

#### Từ source

```bash
git clone https://github.com/LocShadowVN/kestrel-browser.git
cd kestrel-browser
cargo build --release
./target/release/kestrel
```

Lần build đầu tiên sẽ tải SpiderMonkey và compile Servo engine (~15–30 phút).

#### Dependencies (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential curl wget file \
  libssl-dev libgtk-4-dev \
  libx11-dev libxcb1-dev \
  libfontconfig1-dev libfreetype6-dev \
  libharfbuzz-dev libgl1-mesa-dev libegl1-mesa-dev \
  pkg-config
```

#### Dependencies (Fedora)

```bash
sudo dnf install -y \
  gcc gcc-c++ make curl wget file openssl-devel \
  gtk4-devel libX11-devel libxcb-devel \
  fontconfig-devel freetype-devel harfbuzz-devel \
  mesa-libGL-devel mesa-libEGL-devel pkg-config
```

### Biến môi trường

```bash
# Debug log
RUST_LOG=info kestrel

# Chọn render backend
KESTREL_RENDER=vulkan    # Vulkan (default nếu có)
KESTREL_RENDER=gl        # OpenGL fallback
KESTREL_RENDER=software  # Software render (CPU)

# Chọn window backend
KESTREL_WINDOW=x11       # X11 / XWayland
KESTREL_WINDOW=wayland   # Wayland native
```

### Roadmap

#### Phase 1 — Browser cơ bản (đang làm)
- [x] Servo engine embed
- [x] GTK4 chrome
- [x] Multi-tab
- [x] Omnibox + navigation
- [ ] History view
- [ ] Bookmarks view
- [ ] Session restore

#### Phase 2 — Adblock
- [ ] Network hook qua `WebViewDelegate`
- [ ] `adblock-rust` engine integration
- [ ] Cosmetic filter injection
- [ ] Per-site shield toggle

#### Phase 3 — Privacy
- [ ] Vault (Argon2id + AES-256-GCM)
- [ ] WebRTC leak shield
- [ ] UA spoof per-site
- [ ] Clean URL tracking params

#### Phase 4 — Media
- [ ] GStreamer integration test
- [ ] MP4/WebM playback verify
- [ ] MSE support (chờ Servo upstream)

#### Phase 5 — Ecosystem
- [ ] Extension API (WebExtension subset)
- [ ] Sync (self-hosted)
- [ ] Flatpak release
- [ ] AUR package

### Ghi chú kỹ thuật

#### Vị trí dữ liệu

- **Database**: `~/.local/share/kestrel-browser/kestrel.sqlite`
- **Config**: `~/.config/kestrel-browser/config.toml`
- **Cache**: `~/.cache/kestrel-browser/`

#### Adblock — chưa làm

Phase 1 **không có adblock**. Đây là quyết định có chủ đích — để browser chạy
được cơ bản trước, rồi mới thêm tính năng.

Khi làm Phase 2, adblock sẽ hook qua `WebViewDelegate::intercept_web_resource_load`
— API chính thức của Servo, được thiết kế cho mục đích này.

#### Về WebKitGTK

Kestrel không dùng WebKitGTK. Nếu bạn cần WebKitGTK với đầy đủ tính năng
(adblock, vault, download), xem phiên bản cũ **Vibird v2.4.2** — vẫn được giữ
ở tag `v2.4.2-webkit-final`.

### Đóng góp

Dự án đang ở giai đoạn đầu. Mọi đóng góp đều hoan nghênh:

- Báo lỗi render trên site cụ thể
- Test trên hardware khác nhau (Intel / AMD / Nvidia)
- Đóng góp polyfill cho API Servo còn thiếu
- Cải thiện documentation
- Dịch UI sang ngôn ngữ khác

Xem [CONTRIBUTING.md](CONTRIBUTING.md) để biết chi tiết.

### License

GNU General Public License v3.0. Xem [LICENSE](LICENSE).

---

<a name="english"></a>
## English

### Why a rewrite?

The previous version (Vibird) used WebKitGTK through Tauri. It worked — 4-layer
adblock, vault, downloads. But it had one unfixable problem:

**WebKitGTK has compositor bugs on Nvidia + Wayland.** The app could get
OOM-killed or dropped to login screen within minutes. This is a WebKitGTK bug,
not an app bug — it affects every WebKitGTK app (GNOME Web, Devhelp, Yelp).

After evaluating WebKitGTK, CEF, Blitz, Sciter, and Ultralight, only **Servo**
met 3 criteria:
- **Written in Rust** (same ecosystem, easy to contribute)
- **Lightweight and embedding-first** (no 200 MB Chromium bundle)
- **Has a future** (Linux Foundation Europe backed, Igalia-led)

Kestrel is a long-term bet on Servo. Trade-off accepted: lower web compatibility
(78% WPT vs 96%), but escapes the Nvidia + Wayland bugs.

### Goals

1. **Actually lightweight.** Low RAM, small binary, no heavy bundled engine.
2. **No Nvidia bugs.** Servo renders via WebRender + wgpu, no legacy GLX.
3. **Pure Rust.** No C++, no unsafe FFI, easy to read, easy to contribute.
4. **Runs basic web.** Reading news, searching, GitHub, Wikipedia.

**Not goals**: replace Chrome, run YouTube 4K, load Chrome extensions.
Kestrel is a **secondary browser** for simple workflows.

### Status

#### ✅ Working (Phase 1)

| Feature | Notes |
|---|---|
| Servo engine | WebRender-based |
| GTK4 UI chrome | Native, not web-based |
| Multi-tab | GTK4 Notebook |
| Omnibox | URL + search + autocomplete |
| Back / forward / reload | Basic |
| History | SQLite |
| Bookmarks | Add / remove |
| Settings dialog | Search engine, homepage |
| Session restore | Save tabs on exit |
| Keyboard shortcuts | Ctrl+T, Ctrl+W, Ctrl+L, F5, ... |

#### ⚠️ Not yet (Phase 2+)

| Feature | Notes |
|---|---|
| Adblock | None yet. No layer at all. |
| Vault | Not started. |
| Download manager | Not started. |
| Extension support | Not started. |
| Auto-update | Not started. |
| Privacy shields | Not started. |

#### ❌ Not supported

| Feature | Reason |
|---|---|
| YouTube video | Servo lacks MSE |
| Netflix, Spotify Web | No DRM (Widevine) |
| Google Meet, Teams | No WebCodecs |
| Chrome/Firefox extensions | Incompatible |

### Web compatibility (expected)

Based on Servo's WPT pass rate (~78%):

| Site type | Compatibility | Notes |
|---|---|---|
| Static (blogs, news) | ✅ Good | VNExpress, BBC, Wikipedia |
| GitHub | ✅ Good | Renders fine |
| Documentation | ✅ Good | MDN, docs.rs |
| Google Search | ✅ Good | |
| Gmail | ⚠️ Partial | Layout may break |
| YouTube | ❌ Poor | UI loads, video doesn't play |
| Facebook | ⚠️ Partial | Feed loads, widgets broken |
| React/Vue SPAs | ⚠️ Varies | Layout may break |

**Reading news, searching, GitHub** → fine.
**YouTube, Netflix, Google Meet** → not supported.

### Architecture

```
┌───────────────────────────────────────────────────────────┐
│                     GTK4 Application                       │
├───────────────────────────────────────────────────────────┤
│  Main Window                                               │
│    ├── Toolbar  (back/forward/reload + omnibox + menu)    │
│    ├── Tab Bar  (GTK4 Notebook)                           │
│    └── Content Area                                       │
│         └── Servo WebView (per tab)                       │
└───────────────────────────────────────────────────────────┘
                        │
                        ▼
┌───────────────────────────────────────────────────────────┐
│              Servo engine (Rust, native)                   │
│  WebRender · Stylo · SpiderMonkey · servo-media           │
└───────────────────────────────────────────────────────────┘
                        │
                        ▼
┌───────────────────────────────────────────────────────────┐
│                  Kestrel Core (Rust)                       │
│  SQLite · Session · Config                                │
└───────────────────────────────────────────────────────────┘
```

No Tauri layer. No webview-in-webview. GTK4 chrome calls Servo API directly
through Rust function calls.

### Installation

**Requirements:**
- Linux x86_64
- Vulkan or OpenGL 3.3+
- GTK4 (usually available on Ubuntu 22.04+, Fedora 37+)

#### From source

```bash
git clone https://github.com/LocShadowVN/kestrel-browser.git
cd kestrel-browser
cargo build --release
./target/release/kestrel
```

First build downloads SpiderMonkey and compiles Servo (~15–30 min).

### Roadmap

#### Phase 1 — Basic browser (in progress)
- [x] Servo engine embed
- [x] GTK4 chrome
- [x] Multi-tab
- [x] Omnibox + navigation
- [ ] History view
- [ ] Bookmarks view
- [ ] Session restore

#### Phase 2 — Adblock
- [ ] Network hook via `WebViewDelegate`
- [ ] `adblock-rust` engine integration
- [ ] Cosmetic filter injection
- [ ] Per-site shield toggle

#### Phase 3 — Privacy
- [ ] Vault (Argon2id + AES-256-GCM)
- [ ] WebRTC leak shield
- [ ] UA spoof per-site
- [ ] Clean URL tracking params

#### Phase 4 — Media
- [ ] GStreamer integration test
- [ ] MP4/WebM playback verify
- [ ] MSE support (waiting for Servo upstream)

#### Phase 5 — Ecosystem
- [ ] Extension API (WebExtension subset)
- [ ] Sync (self-hosted)
- [ ] Flatpak release
- [ ] AUR package

### Contributing

The project is early-stage. Contributions welcome:

- Report render bugs on specific sites
- Test on different hardware (Intel / AMD / Nvidia)
- Contribute polyfills for missing Servo APIs
- Improve documentation
- Translate UI to other languages

### License

GNU General Public License v3.0. See [LICENSE](LICENSE).
