# Kestrel Browser

A lightweight, pure-Rust web browser for Linux, built on the **Servo engine**.

[![Build](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20x86__64-lightgrey.svg?style=flat-square)]()
[![Engine](https://img.shields.io/badge/engine-Servo-orange.svg?style=flat-square)](https://servo.org/)
[![Status](https://img.shields.io/badge/status-approaching%20alpha-yellow.svg?style=flat-square)](#status)

[English](#english) · [Tiếng Việt](#tiếng-việt)

---

<a name="english"></a>
## English

### Status

**Approaching first alpha release.** Kestrel builds, runs, and renders real
web pages on Linux. The core browser loop — navigation, tabs, internal pages,
settings, and history — is functional. Packaging for end users (`.deb`,
Flatpak) is planned next.

Not yet ready for daily use. See [Known Limitations](#known-limitations).

### About

Kestrel is an experimental desktop browser for Linux, built on the
[Servo engine](https://servo.org) — a pure-Rust browser engine backed by
Linux Foundation Europe and developed primarily by Igalia. The project targets
a browser that runs on modest hardware, does not bundle Chromium, and ships
with zero telemetry.

Kestrel is a long-term bet on Servo. It trades web compatibility for a lighter
runtime footprint and a codebase that is fully auditable in Rust.

### Features

- **GTK4 native UI** — window, tab bar, toolbar, omnibox, hamburger menu
- **Servo engine** — renders HTML, CSS, and full Unicode
- **Navigation** — back, forward, reload, home
- **Omnibox** — enter a URL, or search with Brave Search
- **Internal pages** — `kestrel://home`, `kestrel://settings`, `kestrel://history`
- **Bilingual UI** — English and Vietnamese, switchable in settings
- **Local storage** — SQLite for history, bookmarks, and settings
- **Compat module** — JS polyfills for API gaps in current Servo

### Roadmap

| Phase | Content | Status |
|---|---|---|
| **0** | Stable CI build | Done |
| **1** | Basic working browser | Done |
| **1.5** | Compat module (JS polyfills) | In progress |
| **2** | Adblock (network + cosmetic) via `adblock-rust` | Planned |
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

The binary is produced at `target/release/kestrel`.

### Known Limitations

Servo is under active development. The following are not yet supported:

| Feature | Reason |
|---|---|
| YouTube video | Servo lacks MSE (Media Source Extensions) |
| Netflix, Spotify Web | No DRM (Widevine CDM) |
| Google Meet, Microsoft Teams | No WebCodecs, no full WebRTC |
| Chrome/Firefox extensions | Servo lacks WebExtension API |
| Some SPAs (react.dev) | Missing JS APIs; polyfills insufficient |

These limitations are tracked and will be revisited as Servo upstream
adds support.

### Previous Version — Vibird

If you need a feature-complete browser today (4-layer adblock, vault,
downloads, session), see [LocShadowVN/VibirdBrowser](https://github.com/LocShadowVN/VibirdBrowser)
— the WebKitGTK-based predecessor.

### Contributing

Contributions are welcome. Useful areas:

- Fixing dependency conflicts and CI issues
- Reporting build errors with full logs
- Testing on different GPU and desktop configurations
- Improving documentation and translations

### License

GNU General Public License v3.0. See [LICENSE](LICENSE).

---

<a name="tiếng-việt"></a>
## Tiếng Việt

### Trạng thái

**Sắp phát hành bản alpha đầu tiên.** Kestrel build được, chạy được, và
render được trang web thật trên Linux. Vòng lặp trình duyệt cốt lõi —
điều hướng, tab, trang nội bộ, cài đặt, lịch sử — đã hoạt động. Bước tiếp
theo là đóng gói cho người dùng cuối (`.deb`, Flatpak).

Chưa sẵn sàng dùng hàng ngày. Xem [Giới hạn đã biết](#giới-hạn-đã-biết).

### Giới thiệu

Kestrel là trình duyệt desktop thử nghiệm cho Linux, xây trên
[Servo engine](https://servo.org) — engine trình duyệt thuần Rust do
Linux Foundation Europe bảo trợ và Igalia phát triển chính. Mục tiêu:
một trình duyệt chạy được trên máy cấu hình thấp, không bundle Chromium,
không telemetry.

Kestrel là canh bạc dài hạn vào Servo. Đánh đổi web compatibility lấy
runtime nhẹ hơn và codebase thuần Rust có thể kiểm tra toàn bộ.

### Tính năng hiện có

- **GTK4 native UI** — window, tab bar, toolbar, omnibox, menu hamburger
- **Servo engine** — render HTML, CSS và Unicode đầy đủ
- **Điều hướng** — back, forward, reload, home
- **Omnibox** — nhập URL, hoặc search qua Brave Search
- **Trang nội bộ** — `kestrel://home`, `kestrel://settings`, `kestrel://history`
- **Giao diện song ngữ** — Anh và Việt, đổi trong settings
- **Lưu trữ cục bộ** — SQLite cho lịch sử, bookmark, và cài đặt
- **Compat module** — polyfill JS bù các API Servo còn thiếu

### Roadmap

| Phase | Nội dung | Trạng thái |
|---|---|---|
| **0** | Build ổn định trên CI | Xong |
| **1** | Browser cơ bản chạy được | Xong |
| **1.5** | Compat module (polyfill JS) | Đang làm |
| **2** | Adblock (network + cosmetic) qua `adblock-rust` | Kế hoạch |
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

Binary nằm ở `target/release/kestrel`.

### Giới hạn đã biết

Servo đang phát triển tích cực. Các tính năng sau chưa được hỗ trợ:

| Tính năng | Lý do |
|---|---|
| YouTube video | Servo chưa có MSE (Media Source Extensions) |
| Netflix, Spotify Web | Thiếu DRM (Widevine CDM) |
| Google Meet, Microsoft Teams | Thiếu WebCodecs, WebRTC đầy đủ |
| Extension Chrome/Firefox | Servo chưa có WebExtension API |
| Một số SPA (react.dev) | Thiếu JS API; polyfill chưa đủ |

Danh sách này sẽ được cập nhật khi Servo upstream bổ sung tính năng.

### Phiên bản trước — Vibird

Nếu bạn cần một trình duyệt đầy đủ tính năng ngay bây giờ (adblock 4 tầng,
vault, download, session), xem [LocShadowVN/VibirdBrowser](https://github.com/LocShadowVN/VibirdBrowser)
— phiên bản tiền nhiệm dùng WebKitGTK.

### Đóng góp

Đóng góp được hoan nghênh. Các mảng hữu ích:

- Sửa dependency conflict và vấn đề CI
- Báo cáo lỗi build kèm log đầy đủ
- Test trên các cấu hình GPU và desktop khác nhau
- Cải thiện documentation và bản dịch

### Giấy phép

GNU General Public License v3.0. Xem [LICENSE](LICENSE).
