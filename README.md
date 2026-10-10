# Kestrel Browser

A desktop web browser for Linux, built on the **Servo engine** and written in Rust.

[![Build](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20x86__64-lightgrey.svg?style=flat-square)]()
[![Engine](https://img.shields.io/badge/engine-Servo-orange.svg?style=flat-square)](https://servo.org/)

[Tiếng Việt](#tiếng-việt) · [English](#english)

---

## Overview

Kestrel is a desktop web browser for Linux, built on the Servo engine — a pure-Rust browser engine backed by Linux Foundation Europe and developed primarily by Igalia. The project targets a lightweight, embedding-first browser that runs on modest hardware, does not bundle Chromium, and ships zero telemetry.

**Status:** Working. The browser builds, runs, and renders web pages. Not yet released as a binary package.

---

## Tiếng Việt

### Giới thiệu

Kestrel là trình duyệt desktop cho Linux, xây trên **Servo engine** — engine trình duyệt thuần Rust do Linux Foundation Europe bảo trợ và Igalia phát triển chính. Mục tiêu: một trình duyệt nhẹ, thiết kế cho nhúng, chạy được trên máy cấu hình thấp, không bundle Chromium, không telemetry.

**Trạng thái:** Đã hoạt động. Trình duyệt build được, chạy được, render được trang web. Chưa phát hành binary chính thức.

### Tính năng hiện có

- GTK4 UI: window, toolbar, omnibox, tabbar
- Servo engine render HTML/CSS/Unicode
- Điều hướng: back, forward, reload, home
- Omnibox: nhập URL hoặc search (Brave Search)
- Trang nội bộ: `kestrel://home`, `kestrel://settings`, `kestrel://history`
- SQLite storage: history, bookmarks (schema), settings
- Compat module: polyfill JS cho Servo

### Roadmap

| Phase | Nội dung | Trạng thái |
|---|---|---|
| **Phase 0** | Build ổn định | ✅ Xong |
| **Phase 1** | Browser cơ bản chạy được | ✅ Xong |
| **Phase 1.5** | Compat module (polyfill JS) | 🔧 Đang làm |
| **Phase 2** | Adblock (network + cosmetic) | 📋 Kế hoạch |
| **Phase 3** | Privacy: vault, UA spoofing | 📋 Kế hoạch |
| **Phase 4** | Media: video, audio | ⏸ Phụ thuộc Servo upstream |

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

Do giới hạn của Servo engine tại thời điểm hiện tại:

| Tính năng | Lý do |
|---|---|
| YouTube video | Servo chưa có MSE |
| Netflix, Spotify Web | Thiếu DRM (Widevine CDM) |
| Google Meet, Microsoft Teams | Thiếu WebCodecs, WebRTC đầy đủ |
| Extension Chrome/Firefox | Servo chưa có WebExtension API |
| Một số SPA (react.dev) | Thiếu JS API, polyfill chưa đủ |

### Phiên bản trước — Vibird

Xem [LocShadowVN/VibirdBrowser](https://github.com/LocShadowVN/VibirdBrowser) — trình duyệt WebKitGTK đầy đủ tính năng.

### Giấy phép

GNU General Public License v3.0. Xem [LICENSE](LICENSE).

---

## English

### About

Kestrel is a desktop web browser for Linux, built on the **Servo engine** — a pure-Rust browser engine backed by Linux Foundation Europe and developed primarily by Igalia.

**Status:** Working. The browser builds, runs, and renders web pages. Not yet released.

### Features

- GTK4 UI: window, toolbar, omnibox, tabbar
- Servo engine rendering HTML/CSS/Unicode
- Navigation: back, forward, reload, home
- Omnibox: URL entry or search (Brave Search)
- Internal pages: `kestrel://home`, `kestrel://settings`, `kestrel://history`
- SQLite storage: history, bookmarks (schema), settings
- Compat module: JS polyfills for Servo

### Roadmap

| Phase | Content | Status |
|---|---|---|
| **Phase 0** | Stable build | ✅ Done |
| **Phase 1** | Basic working browser | ✅ Done |
| **Phase 1.5** | Compat module (JS polyfills) | 🔧 In progress |
| **Phase 2** | Adblock (network + cosmetic) | 📋 Planned |
| **Phase 3** | Privacy: vault, UA spoofing | 📋 Planned |
| **Phase 4** | Media: video, audio | ⏸ Blocked by Servo upstream |

### Building from Source

```bash
git clone https://github.com/LocShadowVN/kestrel-browser.git
cd kestrel-browser
cargo build --release
```

### Known Limitations

| Feature | Reason |
|---|---|
| YouTube video | Servo lacks MSE |
| Netflix, Spotify Web | No DRM (Widevine CDM) |
| Google Meet, Microsoft Teams | No WebCodecs, no full WebRTC |
| Chrome/Firefox extensions | Servo lacks WebExtension API |
| Some SPAs (react.dev) | Missing JS APIs, polyfills insufficient |

### Previous Version — Vibird

See [LocShadowVN/VibirdBrowser](https://github.com/LocShadowVN/VibirdBrowser) — full-featured WebKitGTK browser.

### License

GNU General Public License v3.0. See [LICENSE](LICENSE).
