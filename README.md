# Kestrel Browser

A desktop web browser for Linux, built on the **Servo engine** and written in Rust.

[![Build](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20x86__64-lightgrey.svg?style=flat-square)]()
[![Engine](https://img.shields.io/badge/engine-Servo-orange.svg?style=flat-square)](https://servo.org/)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-dea584.svg?style=flat-square)]()

[Tiếng Việt](#tiếng-việt) · [English](#english)

---

## Overview

Kestrel is an experimental desktop browser for Linux, built on the Servo engine — a pure-Rust browser engine backed by Linux Foundation Europe and developed primarily by Igalia. The project targets a lightweight, embedding-first browser that runs on modest hardware, does not bundle Chromium, and ships zero telemetry.

**This is an early-stage project.** The architecture and core modules are in place; the Servo integration and UI layer are under active development.

---

## Tiếng Việt

### Giới thiệu

Kestrel là trình duyệt desktop thử nghiệm cho Linux, xây trên **Servo engine** — engine trình duyệt thuần Rust do Linux Foundation Europe bảo trợ và Igalia phát triển chính. Mục tiêu: một trình duyệt nhẹ, thiết kế cho nhúng, chạy được trên máy cấu hình thấp, không bundle Chromium, không telemetry.

**Dự án đang ở giai đoạn đầu.** Kiến trúc và các module lõi đã có; phần tích hợp Servo và UI đang được hoàn thiện.

### Động lực

Phiên bản trước — **Vibird** — dùng WebKitGTK qua Tauri. Vibird hoạt động ổn định với adblock 4 tầng, vault và download, nhưng gặp một lỗi compositor không thể khắc phục trên cấu hình **Nvidia + Wayland**: ứng dụng có thể bị OOM-kill hoặc văng về login screen sau vài phút. Đây là lỗi của WebKitGTK, ảnh hưởng đến mọi ứng dụng dùng nó (GNOME Web, Devhelp, Yelp).

Sau khi đánh giá các phương án thay thế (WebKitGTK, CEF, Blitz, Sciter, Ultralight), chỉ Servo đáp ứng đủ các tiêu chí:

- Viết bằng Rust
- Nhẹ, thiết kế cho nhúng
- Được phát triển tích cực (Sovereign Tech Fund, Igalia)

Kestrel chấp nhận đánh đổi: web compatibility thấp hơn WebKit (78% WPT so với 96%), nhưng thoát khỏi lỗi Nvidia + Wayland.

### Trạng thái dự án

| Thành phần | Trạng thái | Ghi chú |
|---|---|---|
| Build (CI) | Đang ổn định | Dependency resolution đang được xử lý |
| Binary | Chưa phát hành | Sẽ có sau khi Phase 1 hoàn tất |
| GTK4 UI | Đang phát triển | Window, toolbar, omnibox, tabbar |
| Tích hợp Servo | Đang phát triển | Qua servo-gtk |
| Storage (SQLite) | Hoàn tất | Schema + queries + unit tests |
| Chuẩn hoá URL | Hoàn tất | Có unit tests |
| Compat module | Kế hoạch | Polyfill JS cho Servo |

### Roadmap

| Phase | Nội dung | Trạng thái |
|---|---|---|
| **Phase 0** | Build ổn định trên CI | Đang hoàn thiện |
| **Phase 1** | Browser cơ bản: UI + Servo WebView render được | Chờ Phase 0 |
| **Phase 1.5** | Compat module — polyfill JS cho Servo | Kế hoạch |
| **Phase 2** | Adblock (network + cosmetic) qua adblock-rust | Kế hoạch |
| **Phase 3** | Privacy: vault, UA spoofing | Kế hoạch |
| **Phase 4** | Media: video, audio | Phụ thuộc Servo upstream |

### Kiến trúc

Cấu trúc thư mục của dự án:

- `shared/` — Kiểu dữ liệu chia sẻ (serde)
- `src-servo/` — Binary chính
  - `app.rs` — Vòng đời ứng dụng GTK
  - `chrome/` — UI: window, toolbar, omnibox, tabbar
  - `engine/` — Chuẩn hoá URL, navigation
  - `storage/` — SQLite: history, bookmarks, settings
  - `util/` — Config, logging
- `.github/workflows/` — CI build

**Thành phần kỹ thuật:**

- **Engine:** Servo, qua wrapper servo-gtk (GTK4 widget)
- **UI:** GTK4
- **Storage:** SQLite qua rusqlite (bundled)
- **Adblock (kế hoạch):** adblock-rust — engine filter của Brave, tương thích EasyList, EasyPrivacy

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

Binary: `target/release/kestrel`.

### Giới hạn đã biết

Do giới hạn của Servo engine tại thời điểm hiện tại, các tính năng sau **chưa được hỗ trợ**:

| Tính năng | Lý do |
|---|---|
| YouTube video | Servo chưa có MSE (Media Source Extensions) |
| Netflix, Spotify Web | Thiếu DRM (Widevine CDM) |
| Google Meet, Microsoft Teams | Thiếu WebCodecs, WebRTC đầy đủ |
| Extension Chrome/Firefox | Servo chưa có WebExtension API |
| Google Docs editor | Thiếu một số DOM API phức tạp |

Những giới hạn này sẽ được cập nhật khi Servo upstream bổ sung tính năng.

### Phiên bản trước — Vibird

Nếu cần một trình duyệt đầy đủ tính năng ngay bây giờ (adblock 4 tầng, vault, download, session), xem phiên bản cũ **Vibird** tại [LocShadowVN/VibirdBrowser](https://github.com/LocShadowVN/VibirdBrowser).

**Lưu ý:** Vibird có lỗi Nvidia + Wayland đã nêu ở trên. Trên Intel/AMD iGPU và X11, nó chạy ổn định.

### Đóng góp

Dự án đang ở giai đoạn đầu. Đóng góp được hoan nghênh, đặc biệt ở các mảng:

- Sửa dependency conflicts
- Báo cáo lỗi build (kèm log)
- Test trên các cấu hình GPU/desktop khác nhau
- Cải thiện documentation

### Giấy phép

GNU General Public License v3.0. Xem [LICENSE](LICENSE).

---

## English

### About

Kestrel is an experimental desktop browser for Linux, built on the **Servo engine** — a pure-Rust browser engine backed by Linux Foundation Europe and developed primarily by Igalia. The long-term goal is a lightweight, embedding-first browser that runs on modest hardware, does not bundle Chromium, and ships zero telemetry.

**The project is at an early stage.** The architecture and core modules are in place; Servo integration and the UI layer are under active development.

### Motivation

The previous version — **Vibird** — used WebKitGTK through Tauri. Vibird worked reliably with a 4-layer adblock, vault, and download support, but suffered from an unfixable compositor bug on **Nvidia + Wayland**: the app could be OOM-killed or dropped to the login screen within minutes. This is a WebKitGTK bug affecting every WebKitGTK application (GNOME Web, Devhelp, Yelp).

After evaluating alternatives (WebKitGTK, CEF, Blitz, Sciter, Ultralight), only Servo met the criteria:

- Written in Rust
- Lightweight, embedding-first
- Actively developed (Sovereign Tech Fund, Igalia)

Kestrel accepts the trade-off: lower web compatibility (78% WPT vs 96%), but escapes the Nvidia + Wayland bugs.

### Project Status

| Component | Status | Notes |
|---|---|---|
| Build (CI) | Stabilizing | Dependency resolution in progress |
| Binary | Not released | Will ship after Phase 1 |
| GTK4 UI | In development | Window, toolbar, omnibox, tabbar |
| Servo integration | In development | Via servo-gtk |
| Storage (SQLite) | Complete | Schema + queries + unit tests |
| URL normalization | Complete | Unit tested |
| Compat module | Planned | JS polyfills for Servo |

### Roadmap

| Phase | Content | Status |
|---|---|---|
| **Phase 0** | Stable CI build | In progress |
| **Phase 1** | Basic browser: UI + Servo WebView rendering | Blocked by Phase 0 |
| **Phase 1.5** | Compat module — JS polyfills for Servo | Planned |
| **Phase 2** | Adblock (network + cosmetic) via adblock-rust | Planned |
| **Phase 3** | Privacy: vault, UA spoofing | Planned |
| **Phase 4** | Media: video, audio | Blocked by Servo upstream |

### Architecture

Project directory structure:

- `shared/` — Shared data types (serde)
- `src-servo/` — Main binary
  - `app.rs` — GTK application lifecycle
  - `chrome/` — UI: window, toolbar, omnibox, tabbar
  - `engine/` — URL normalization, navigation
  - `storage/` — SQLite: history, bookmarks, settings
  - `util/` — Config, logging
- `.github/workflows/` — CI build

**Technical stack:**

- **Engine:** Servo, via the servo-gtk GTK4 widget wrapper
- **UI:** GTK4
- **Storage:** SQLite via rusqlite (bundled)
- **Adblock (planned):** adblock-rust — the filter engine used by Brave, compatible with EasyList and EasyPrivacy

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

Due to current Servo engine limitations, the following features are **not supported**:

| Feature | Reason |
|---|---|
| YouTube video | Servo lacks MSE (Media Source Extensions) |
| Netflix, Spotify Web | No DRM (Widevine CDM) |
| Google Meet, Microsoft Teams | No WebCodecs, no full WebRTC |
| Chrome/Firefox extensions | Servo lacks WebExtension API |
| Google Docs editor | Missing complex DOM APIs |

These limitations will be revisited as Servo upstream adds support.

### Previous Version — Vibird

For a feature-complete browser available today (4-layer adblock, vault, downloads, session), see the previous **Vibird** at [LocShadowVN/VibirdBrowser](https://github.com/LocShadowVN/VibirdBrowser).

**Note:** Vibird has the Nvidia + Wayland bug described above. On Intel/AMD iGPU with X11, it runs stably.

### Contributing

The project is at an early stage. Contributions are welcome, especially:

- Fixing dependency conflicts
- Reporting build errors (with logs)
- Testing on different GPU/desktop configurations
- Improving documentation

### License

GNU General Public License v3.0. See [LICENSE](LICENSE).
