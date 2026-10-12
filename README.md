# Kestrel Browser

A lightweight, pure-Rust web browser for Linux, built on the **Servo engine**.

[![Build](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20x86__64-lightgrey.svg?style=flat-square)]()
[![Engine](https://img.shields.io/badge/engine-Servo-orange.svg?style=flat-square)](https://servo.org/)
[![Status](https://img.shields.io/badge/status-beta-orange.svg?style=flat-square)](#status)

[English](#english) · [Tiếng Việt](#tiếng-việt)

---

<a name="english"></a>
## English

### Status

**Beta.** Kestrel builds, runs, and renders real web pages on Linux. Core
browser features — navigation, tabs, internal pages, settings, history,
bookmarks, and network-level ad blocking — are functional.

Not yet ready for daily driver use (see [Known Limitations](#known-limitations)),
but approaching a usable alpha release.

### Features

- **GTK4 native UI** — tab strip, toolbar, omnibox, hamburger menu
- **Servo engine** — renders HTML, CSS, and full Unicode
- **Navigation** — back, forward, reload, home
- **Omnibox** — enter a URL or search with Brave Search
- **Internal pages** — `kestrel://home`, `kestrel://settings`,
  `kestrel://history`, `kestrel://bookmarks`
- **Bilingual UI** — English and Vietnamese, switchable in settings
- **Local storage** — SQLite for history, bookmarks, and settings
- **Ad blocking** — network-level, powered by
  [`adblock-rust`](https://github.com/brave/adblock-rust) (Brave's engine)
  with EasyList + EasyPrivacy + Fanboy's Annoyances. Three levels:
  Off / Standard / Aggressive.
- **Web compat module** — JS polyfills for API gaps in current Servo

### Architecture

Kestrel depends on a **fork of
[`nacho/servo-gtk`](https://github.com/nacho/servo-gtk)** at
[`LocShadowVN/servo-gtk`](https://github.com/LocShadowVN/servo-gtk). The fork
adds a `WebViewDelegate::load_web_resource` hook so subresource requests can
be inspected and cancelled before Servo dispatches them. The adblock engine
runs inside the Servo runner subprocess, so per-request filtering is a plain
function call rather than an IPC round-trip.

```
kestrel-browser/
├── shared/              # Shared types (serde)
├── src-servo/           # Main binary
│   ├── adblock_rules.rs # Filter lists, embedded at build time
│   ├── build.rs         # Downloads and embeds EasyList etc.
│   ├── chrome/          # UI (tabs, toolbar, omnibox, adblock button)
│   ├── engine/          # URL normalization
│   ├── internal_pages/  # kestrel://home, settings, history, bookmarks
│   ├── storage/         # SQLite
│   └── util/            # Config, logging
└── .github/workflows/   # CI + release
```

### Roadmap

| Phase | Content | Status |
|---|---|---|
| **0** | Stable CI build | Done |
| **1** | Basic working browser | Done |
| **1.5** | Compat module (JS polyfills) | Done |
| **2** | Ad blocking via `adblock-rust` | Done |
| **3** | Privacy: vault, UA spoofing | Planned |
| **4** | Media: video, audio | Blocked by Servo upstream |
| **5** | Packaging: `.deb`, Flatpak, AUR | Planned |

### Requirements

- **OS:** Linux x86_64
- **GPU:** Vulkan or OpenGL 3.3+
- **GTK4:** available on Ubuntu 22.04+, Fedora 37+, Arch
- **RAM:** 4 GB minimum, 8 GB recommended for build

### Building from Source

```bash
git clone https://github.com/LocShadowVN/kestrel-browser.git
cd kestrel-browser
cargo build --release
```

The binary is produced at `target/release/kestrel`. Filter lists
(EasyList, EasyPrivacy, Fanboy's Annoyances) are downloaded by the build
script on first build and embedded into the binary; subsequent builds reuse
the cache.

### Known Limitations

Servo is under active development. The following are not yet supported:

| Feature | Reason |
|---|---|
| YouTube video | Servo lacks MSE (Media Source Extensions) |
| Netflix, Spotify Web | No DRM (Widevine CDM) |
| Google Meet, Microsoft Teams | No WebCodecs, no full WebRTC |
| Chrome/Firefox extensions | Servo lacks WebExtension API |
| Some SPAs (react.dev) | Missing JS APIs; polyfills insufficient |

### Previous Version — Vibird

If you need a feature-complete browser today (4-layer adblock, vault,
downloads, session), see
[LocShadowVN/VibirdBrowser](https://github.com/LocShadowVN/VibirdBrowser) —
the WebKitGTK-based predecessor.

### Contributing

Contributions are welcome. Useful areas:

- Fixing dependency conflicts and CI issues
- Reporting build errors with full logs
- Testing on different GPU and desktop configurations
- Improving documentation and translations

### Credits

- The Servo engine is developed by [Igalia](https://www.igalia.com) and
  contributors, backed by Linux Foundation Europe.
- [`servo-gtk`](https://github.com/nacho/servo-gtk) upstream is written and
  maintained by [Ignacio Casal Quinteiro (nacho)](https://github.com/nacho).
  Kestrel uses a downstream fork with ad-blocking support.

### License

GNU General Public License v3.0. See [LICENSE](LICENSE).

---

<a name="tiếng-việt"></a>
## Tiếng Việt

### Trạng thái

**Beta.** Kestrel build được, chạy được, và render được trang web thật trên
Linux. Các tính năng cốt lõi — điều hướng, tab, trang nội bộ, cài đặt, lịch
sử, bookmark, và chặn quảng cáo ở tầng network — đã hoạt động.

Chưa sẵn sàng dùng hàng ngày (xem [Giới hạn đã biết](#giới-hạn-đã-biết)),
nhưng đang tiến gần đến bản alpha dùng được.

### Tính năng

- **GTK4 native UI** — tab strip, toolbar, omnibox, menu hamburger
- **Servo engine** — render HTML, CSS và Unicode đầy đủ
- **Điều hướng** — back, forward, reload, home
- **Omnibox** — nhập URL hoặc search qua Brave Search
- **Trang nội bộ** — `kestrel://home`, `kestrel://settings`,
  `kestrel://history`, `kestrel://bookmarks`
- **Giao diện song ngữ** — Anh và Việt, đổi trong settings
- **Lưu trữ cục bộ** — SQLite cho lịch sử, bookmark, và cài đặt
- **Chặn quảng cáo** — ở tầng network, dùng
  [`adblock-rust`](https://github.com/brave/adblock-rust) (engine của Brave)
  với EasyList + EasyPrivacy + Fanboy's Annoyances. Ba mức:
  Off / Standard / Aggressive.
- **Compat module** — polyfill JS bù các API Servo còn thiếu

### Kiến trúc

Kestrel phụ thuộc vào **bản fork của
[`nacho/servo-gtk`](https://github.com/nacho/servo-gtk)** tại
[`LocShadowVN/servo-gtk`](https://github.com/LocShadowVN/servo-gtk). Bản fork
thêm hook `WebViewDelegate::load_web_resource` để kiểm tra và hủy các
subresource request trước khi Servo gửi chúng đi. Engine adblock chạy bên
trong subprocess Servo runner, nên việc lọc mỗi request chỉ là một lời gọi
hàm — không phải IPC round-trip.

```
kestrel-browser/
├── shared/              # Kiểu dữ liệu chia sẻ (serde)
├── src-servo/           # Binary chính
│   ├── adblock_rules.rs # Filter list, nhúng vào binary lúc build
│   ├── build.rs         # Tải và nhúng EasyList, v.v.
│   ├── chrome/          # UI (tabs, toolbar, omnibox, nút adblock)
│   ├── engine/          # Chuẩn hoá URL
│   ├── internal_pages/  # kestrel://home, settings, history, bookmarks
│   ├── storage/         # SQLite
│   └── util/            # Config, logging
└── .github/workflows/   # CI + release
```

### Roadmap

| Phase | Nội dung | Trạng thái |
|---|---|---|
| **0** | Build ổn định trên CI | Xong |
| **1** | Browser cơ bản chạy được | Xong |
| **1.5** | Compat module (polyfill JS) | Xong |
| **2** | Chặn quảng cáo qua `adblock-rust` | Xong |
| **3** | Privacy: vault, UA spoofing | Kế hoạch |
| **4** | Media: video, audio | Phụ thuộc Servo upstream |
| **5** | Đóng gói: `.deb`, Flatpak, AUR | Kế hoạch |

### Yêu cầu hệ thống

- **OS:** Linux x86_64
- **GPU:** Vulkan hoặc OpenGL 3.3+
- **GTK4:** có sẵn trên Ubuntu 22.04+, Fedora 37+, Arch
- **RAM:** 4 GB tối thiểu, 8 GB khuyến nghị khi build

### Build từ source

```bash
git clone https://github.com/LocShadowVN/kestrel-browser.git
cd kestrel-browser
cargo build --release
```

Binary nằm ở `target/release/kestrel`. Filter list (EasyList, EasyPrivacy,
Fanboy's Annoyances) được build script tải về ở lần build đầu tiên và nhúng
vào binary; các lần build sau dùng lại cache.

### Giới hạn đã biết

Servo đang phát triển tích cực. Các tính năng sau chưa được hỗ trợ:

| Tính năng | Lý do |
|---|---|
| YouTube video | Servo chưa có MSE (Media Source Extensions) |
| Netflix, Spotify Web | Thiếu DRM (Widevine CDM) |
| Google Meet, Microsoft Teams | Thiếu WebCodecs, WebRTC đầy đủ |
| Extension Chrome/Firefox | Servo chưa có WebExtension API |
| Một số SPA (react.dev) | Thiếu JS API; polyfill chưa đủ |

### Phiên bản trước — Vibird

Nếu bạn cần một trình duyệt đầy đủ tính năng ngay bây giờ (adblock 4 tầng,
vault, download, session), xem
[LocShadowVN/VibirdBrowser](https://github.com/LocShadowVN/VibirdBrowser)
— phiên bản tiền nhiệm dùng WebKitGTK.

### Đóng góp

Đóng góp được hoan nghênh. Các mảng hữu ích:

- Sửa dependency conflict và vấn đề CI
- Báo cáo lỗi build kèm log đầy đủ
- Test trên các cấu hình GPU và desktop khác nhau
- Cải thiện documentation và bản dịch

### Ghi công

- Engine Servo được phát triển bởi [Igalia](https://www.igalia.com) và
  cộng đồng, với sự bảo trợ của Linux Foundation Europe.
- [`servo-gtk`](https://github.com/nacho/servo-gtk) upstream được viết và
  duy trì bởi [Ignacio Casal Quinteiro (nacho)](https://github.com/nacho).
  Kestrel dùng bản fork hạ nguồn có hỗ trợ chặn quảng cáo.

### Giấy phép

GNU General Public License v3.0. Xem [LICENSE](LICENSE).
