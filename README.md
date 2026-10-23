# Kestrel Browser

**A lightweight, pure-Rust, zero-telemetry web browser for Linux — built on the Servo 0.6 engine.**

[![CI Build](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml)
[![Release](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/release.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/releases)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Platform: Linux x86_64](https://img.shields.io/badge/platform-Linux%20x86__64-lightgrey.svg?style=flat-square)](#system-requirements)
[![Engine: Servo 0.6](https://img.shields.io/badge/engine-Servo%200.6-orange.svg?style=flat-square)](https://servo.org/)
[![Adblock: Brave adblock-rust](https://img.shields.io/badge/shields-adblock--rust-black.svg?style=flat-square)](https://github.com/brave/adblock-rust)

[**English**](#english) · [**Tiếng Việt**](#tiếng-việt)

</div>

---

<a name="english"></a>
## English

### Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Keyboard Shortcuts](#keyboard-shortcuts)
- [Architecture](#architecture)
- [Web Compatibility Layer (Polyfills v5)](#web-compatibility-layer-polyfills-v5)
- [System Requirements](#system-requirements)
- [Installation & Building from Source](#installation--building-from-source)
- [Roadmap](#roadmap)
- [Known Limitations](#known-limitations)
- [Contributing](#contributing)
- [Credits & License](#credits--license)

---

### Overview

**Kestrel** is an independent, privacy-first Linux web browser written entirely in **Rust**. It pairs a clean, responsive **GTK4** native interface with the **[Servo 0.6](https://servo.org/)** layout engine (featuring native CSS Grid support and WebRender GPU rendering) and **[Brave's `adblock-rust`](https://github.com/brave/adblock-rust)** filtering engine.

Designed for speed, low memory overhead, and strict local privacy, Kestrel runs with zero telemetry, zero background cloud services, and dual-layer ad & tracker blocking.

---

### Key Features

- **Refined GTK4 Native Chrome**:
  - Custom monochrome vector SVG icon system with automatic `gdk::Texture` caching and adaptive Dark/Light contrast filtering.
  - Scrollable pill tab strip supporting middle-click and `Ctrl+W` tab closing, live loading indicators, and window title synchronization.
  - Instant **Dark / Light theme** switching and **Bilingual UI** (English & Vietnamese).
  - Smooth 2px page-loading progress bar.
- **Unified Composite Omnibox**:
  - Real-time connection security indicator (`HTTPS` lock, `HTTP` warning, `Kestrel` internal scheme badge).
  - Integrated **Bookmark Star (`Ctrl+D`)** and **Kestrel Shields Badge** directly inside the address bar.
  - Smart URL/domain/IPv4/IPv6/localhost normalization and customizable search engine routing (Brave Search, DuckDuckGo, Google, Bing, or custom query URL).
- **Kestrel Shields (Dual-Layer Ad & Tracker Blocking)**:
  - **Layer 1 — Zero-IPC Network Filtering**: Powered by [`adblock-rust`](https://github.com/brave/adblock-rust) with **EasyList**, **EasyPrivacy**, and **Fanboy's Annoyances** running directly inside the Servo runner subprocess.
  - **Layer 2 — Active Cosmetic & DOM Ad Scrubber**: Injects cosmetic CSS rules and actively collapses empty ad placeholders, inline-height ad wrappers, and third-party ad iframes/scripts (Google Ads, DoubleClick, Eclick, Admicro, Taboola, Outbrain, MGID).
  - **Optimized Binary Filter Cache (`adblock_net_v3.blob`)**: Strips redundant rules at compile time to reduce blob size by ~75% and writes atomically to disk for sub-millisecond warm startups.
  - **Three Protection Modes**: **Off**, **Standard**, and **Aggressive**, with live per-page and session-total blocked counters.
- **Hardware GPU Acceleration**:
  - Automatic GPU driver optimization before subprocess initialization (NVIDIA PRIME Render Offload, `__GL_THREADED_OPTIMIZATIONS`, shader disk caching, and Mesa `mesa_glthread` + hybrid `DRI_PRIME` detection).
- **Built-in Internal Pages (`kestrel://`)**:
  - `kestrel://home` — Privacy dashboard displaying live Ads & Trackers Blocked, estimated bandwidth saved, bookmark/history counters, and quick-access tiles.
  - `kestrel://bookmarks` — Searchable bookmark manager with automatic URL normalization, XSS protection, and instant DOM updates.
  - `kestrel://history` — Searchable browsing history with consecutive URL deduplication, single-entry deletion, and one-click clearing.
  - `kestrel://settings` — Instant-apply segmented controls and interactive cards for theme, language, Shields level, homepage, and search engine presets.
  - `kestrel://about` — Engine, repository, and license overview.
- **Local SQLite WAL Storage**:
  - All settings, bookmarks, and browsing history are persisted locally in `~/.local/share/kestrel-browser/kestrel.sqlite` using SQLite `WAL` (Write-Ahead Logging) mode for fast, non-blocking writes.

---

### Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl + T` | Open a new tab and focus the Omnibox |
| `Ctrl + W` / `Middle-Click` | Close the current tab |
| `Ctrl + L` / `Alt + D` | Focus and select the Omnibox address bar |
| `/` *(on `kestrel://home`)* | Focus the New Tab search bar |
| `Ctrl + R` / `F5` | Reload the current page (or refresh `kestrel://` internal page) |
| `Alt + Left` | Navigate back |
| `Alt + Right` | Navigate forward |
| `Alt + Home` | Navigate to Homepage |
| `F11` | Toggle fullscreen mode |
| `Ctrl + D` | Bookmark or unbookmark the current page |
| `Ctrl + B` | Open Bookmarks (`kestrel://bookmarks`) |
| `Ctrl + H` | Open Browsing History (`kestrel://history`) |
| `Ctrl + ,` | Open Settings (`kestrel://settings`) |
| `Ctrl + Tab` / `Ctrl + PageDown` | Switch to the next tab |
| `Ctrl + Shift + Tab` / `Ctrl + PageUp` | Switch to the previous tab |
| `Ctrl + 1` … `Ctrl + 8` | Switch to tab 1 … 8 |
| `Ctrl + 9` | Switch to the last tab |
| `Ctrl + Q` | Quit Kestrel cleanly |

---

### Architecture

Kestrel uses a downstream fork of [`nacho/servo-gtk`](https://github.com/nacho/servo-gtk) maintained at [`LocShadowVN/servo-gtk`](https://github.com/LocShadowVN/servo-gtk). The fork embeds **Servo 0.6** and exposes a `WebViewDelegate::load_web_resource` hook inside the Servo runner subprocess.

Because the `adblock-rust` engine is deserialized directly inside the runner subprocess from a pre-compiled binary blob (`~/.local/share/kestrel-browser/adblock_net_v3.blob`), every subresource request is inspected via an in-process function call without IPC round-trip latency.

```text
┌──────────────────────────────────────────────────────────────────────┐
│ Main GTK4 Process (kestrel)                                          │
│  ├── Chrome UI (TabBar, Toolbar, Omnibox, Kestrel Shields, Popovers) │
│  ├── SQLite WAL Storage (kestrel.sqlite: History, Bookmarks, Config) │
│  └── Network Adblock Compiler & Atomic Cache (adblock_net_v3.blob)   │
└───────────────┬──────────────────────────────────────▲───────────────┘
                │ IPC (Commands, Blob, UserScripts)    │ Events (URI, Title, Blocked Hits)
┌───────────────▼──────────────────────────────────────┴───────────────┐
│ Servo Runner Subprocess (servo-gtk · Servo 0.6)                      │
│  ├── Servo Layout (CSS Grid), WebRender GPU & SpiderMonkey JS        │
│  ├── WebViewDelegate::load_web_resource ──► adblock-rust Engine      │
│  └── UserContentManager (Polyfills v5 & Active DOM Ad Scrubber)      │
└──────────────────────────────────────────────────────────────────────┘
```

#### Repository Structure

```text
kestrel-browser/
├── shared/                     # Shared data models (HistoryRecord, BookmarkRecord, AppConfig)
├── src-servo/                  # Main browser binary crate
│   ├── build.rs                # Downloads & caches filter lists at build time
│   └── src/
│       ├── main.rs             # Entry point, GPU env setup & Servo runner bootstrap
│       ├── app.rs              # GTK4 Application lifecycle
│       ├── adblock_rules.rs    # Network rule filtering & atomic binary blob caching
│       ├── chrome/             # Native GTK4 UI components
│       │   ├── icons.rs        # Cached monochrome vector SVG icon system
│       │   ├── style.rs        # Global CSS (Adaptive Dark & Light themes)
│       │   ├── window.rs       # MainWindow, tab state machine, IPC & shortcuts
│       │   ├── tabbar.rs       # Scrollable tab strip & dynamic page tracking
│       │   ├── tab_widget.rs   # Individual tab widget with middle-click close
│       │   ├── toolbar.rs      # Navigation bar, progress bar & application menu
│       │   ├── omnibox.rs      # Security badge, URL entry, star & Shields container
│       │   └── adblock_button.rs # Kestrel Shields button & live counter popover
│       ├── compat/
│       │   └── polyfills.rs    # Polyfills v5 + Active Cosmetic/DOM Ad Scrubber
│       ├── engine/
│       │   └── navigation.rs   # URL normalization, local host detection & search routing
│       ├── internal_pages/     # kestrel://home, bookmarks, history, settings
│       ├── storage/
│       │   └── database.rs     # SQLite WAL persistence layer
│       └── util/               # In-memory config cache & logging
└── .github/workflows/          # CI build & automated .deb release pipelines
```

---

### Web Compatibility Layer (Polyfills v5)

While **Servo 0.6** natively supports CSS Grid, `@property`, `touch-action`, and `animation-composition`, some modern JavaScript and DOM APIs expected by production SPAs (React, Next.js, Vue, Svelte, Radix UI) are still evolving upstream. Kestrel injects a strictly **feature-detected** (`typeof ... === 'undefined'`) compatibility layer via `UserContentManager` that never overrides native Servo implementations:

- **Active Cosmetic & DOM Ad Scrubber**: Injects cosmetic filter rules and collapses empty ad containers and third-party ad iframes/scripts while reporting blocked elements back to Kestrel Shields.
- **Deduplicated Layout & Viewport Observers**: Lightweight `ResizeObserver`, `IntersectionObserver` (enables lazy-loaded images and infinite scroll without layout thrashing), `PerformanceObserver`, and `window.visualViewport`.
- **Deep Cloning**: Spec-compliant `structuredClone()` supporting `Date`, `RegExp`, `Map`, `Set`, `ArrayBuffer`, `DataView`, `TypedArray`, and cyclic references via `WeakMap`.
- **Web Animations & DOM**: `Element.prototype.animate()`, `getAnimations()`, `checkVisibility()`, `CSS.supports()`, and `window.matchMedia`.
- **ES2022–ES2024 Built-ins**:
  - Change-Array-by-Copy: `Array.prototype.toSorted()`, `toReversed()`, `toSpliced()`, `with()`.
  - Grouping & Indexing: `Object.hasOwn()`, `Object.fromEntries()`, `Object.groupBy()`, `Map.groupBy()`, `Array/String/TypedArray.prototype.at()`, `findLast()`, `findLastIndex()`.
  - ES2024 Set Operations: `union()`, `intersection()`, `difference()`.
  - Async & Utilities: `Promise.withResolvers()`, `Promise.allSettled()`, `Promise.any()`, `AggregateError`, `queueMicrotask()`, `requestIdleCallback()`, `URL.canParse()`, `crypto.randomUUID()`.

---

### System Requirements

- **OS:** Linux `x86_64` (Ubuntu 22.04+, Debian 12+, Fedora 38+, Arch Linux)
- **GPU:** Hardware OpenGL 3.3+ or Vulkan (Intel, AMD Mesa, or NVIDIA proprietary)
- **GTK4:** `libgtk-4-1` (`>= 4.12`)
- **RAM:** 4 GB minimum for runtime; 8 GB+ recommended when compiling Servo from source

---

### Installation & Building from Source

#### Option 1: Install `.deb` Package (Ubuntu / Debian)

Download the latest `.deb` package from [GitHub Releases](https://github.com/LocShadowVN/kestrel-browser/releases) and install it:

```bash
sudo apt install ./kestrel_*.deb
kestrel
```

#### Option 2: Build from Source

1. **Install system dependencies (Ubuntu / Debian):**

```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential cmake ninja-build python3 python3-pip \
  pkg-config curl wget file clang llvm lld \
  libssl-dev libgtk-4-dev libepoxy-dev \
  libx11-dev libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libfontconfig1-dev libfreetype6-dev libharfbuzz-dev \
  libgl1-mesa-dev libegl1-mesa-dev libgles2-mesa-dev \
  libgbm-dev libwayland-dev libxkbcommon-dev \
  libsqlite3-dev protobuf-compiler
```

2. **Clone and compile:**

```bash
git clone https://github.com/LocShadowVN/kestrel-browser.git
cd kestrel-browser
cargo build --release
```

The release binary is generated at `target/release/kestrel`.

3. **Run unit tests:**

```bash
cargo test --workspace
```

> **Build Environment Variables:**
> - `KESTREL_REQUIRE_FILTERS=1`: Fail the build if downloading EasyList/EasyPrivacy/Fanboy filter lists fails (used in release CI).
> - `KESTREL_REFRESH_FILTERS=1`: Force re-downloading filter lists even if cached in `$OUT_DIR/kestrel_filters/`.

---

### Roadmap

| Phase | Milestone | Status |
|---|---|---|
| **Phase 0** | Reproducible CI build & dependency pinning | Completed |
| **Phase 1** | Core GTK4 browser, tabs, SQLite WAL storage & internal pages | Completed |
| **Phase 1.5** | Web Compatibility JS/DOM Polyfill Suite (v5) & Servo 0.6 | Completed |
| **Phase 2** | Dual-layer Adblocking (`adblock-rust` + DOM Scrubber) & Shields UI | Completed |
| **Phase 3** | Privacy & UX: Session restore, Find in Page (`Ctrl+F`), Page Zoom | Planned |
| **Phase 4** | Rich Media: MSE video streaming, WebCodecs, WebRTC | Upstream Servo |
| **Phase 5** | Packaging: Automated `.deb` release, Flatpak, Arch AUR | `.deb` Done / Flatpak Planned |

---

### Known Limitations

Because Servo is under active upstream development, certain web platform features are not yet available at the engine level:

| Feature | Technical Reason |
|---|---|
| YouTube HD adaptive streaming | Servo lacks complete MSE (Media Source Extensions) |
| Netflix, Spotify Web | No Widevine CDM / EME DRM support |
| Google Meet, Microsoft Teams | Incomplete WebRTC & WebCodecs stack |
| Chrome / Firefox Extensions | Servo does not yet implement the WebExtensions API |

---

### Contributing

Contributions, bug reports, and pull requests are welcome! High-impact areas include:
- Testing across Linux distributions, Wayland/X11 compositors, and hybrid GPU configurations.
- Expanding spec-compliant shims and cosmetic filter rules in `src-servo/src/compat/polyfills.rs`.
- Implementing Phase 3 features (session restore, download manager, find-in-page).

---

### Credits & License

- **Servo Engine** is developed by [Igalia](https://www.igalia.com/) and the Servo community under [Linux Foundation Europe](https://linuxfoundation.eu/).
- **[`servo-gtk`](https://github.com/nacho/servo-gtk)** upstream is created and maintained by [Ignacio Casal Quinteiro (nacho)](https://github.com/nacho).
- **[`adblock-rust`](https://github.com/brave/adblock-rust)** is developed by [Brave Software](https://brave.com/).

Licensed under the **GNU General Public License v3.0**. See [LICENSE](LICENSE) for full details.

---
---

<a name="tiếng-việt"></a>
## Tiếng Việt

### Mục lục

- [Tổng quan](#tổng-quan)
- [Tính năng nổi bật](#tính-năng-nổi-bật)
- [Tổ hợp phím tắt](#tổ-hợp-phím-tắt)
- [Kiến trúc hệ thống](#kiến-trúc-hệ-thống)
- [Tầng tương thích Web (Polyfills v5)](#tầng-tương-thích-web-polyfills-v5)
- [Yêu cầu hệ thống](#yêu-cầu-hệ-thống)
- [Cài đặt & Build từ Source](#cài-đặt--build-từ-source)
- [Lộ trình phát triển (Roadmap)](#lộ-trình-phát-triển-roadmap)
- [Giới hạn đã biết](#giới-hạn-đã-biết)
- [Đóng góp](#đóng-góp)
- [Ghi nhận & Giấy phép](#ghi-nhận--giấy-phép)

---

### Tổng quan

**Kestrel** là trình duyệt web độc lập, nhẹ, thuần **Rust** và không thu thập dữ liệu (zero-telemetry) dành cho Linux. Kestrel kết hợp giao diện **GTK4** tối giản, phản hồi tức thì với nhân trình duyệt thế hệ mới **[Servo 0.6](https://servo.org/)** (hỗ trợ sẵn CSS Grid, tăng tốc phần cứng WebRender GPU) và bộ máy chặn quảng cáo **[`adblock-rust`](https://github.com/brave/adblock-rust)** của Brave.

---

### Tính năng nổi bật

- **Giao diện GTK4 Tối giản & Tinh tế**:
  - Hệ thống biểu tượng vector SVG đơn sắc có bộ nhớ đệm `gdk::Texture` giúp render mượt mà, tự động điều chỉnh độ tương phản sắc nét ở cả **Chế độ Tối (Dark Mode)** và **Chế độ Sáng (Light Mode)**.
  - Thanh tab cuộn ngang mượt mà, hỗ trợ đóng tab bằng **chuột giữa (Middle-click)** hoặc `Ctrl+W`, đồng bộ tiêu đề cửa sổ theo thời gian thực.
  - Hỗ trợ song ngữ **Tiếng Anh & Tiếng Việt** chuyển đổi trực tiếp không cần khởi động lại.
  - Thanh tiến trình tải trang (Progress Bar 2px) mượt mà.
- **Thanh địa chỉ Omnibox hợp nhất**:
  - Biểu tượng trạng thái bảo mật trực quan (Khóa `HTTPS`, cảnh báo `HTTP`, huy hiệu trang nội bộ `Kestrel`).
  - Tích hợp **Nút ngôi sao Bookmark (`Ctrl+D`)** và **Khiên chắn Kestrel Shields** ngay bên trong thanh địa chỉ.
  - Tự động phân giải tên miền, `localhost`, IPv4/IPv6 và định tuyến tìm kiếm thông minh (Brave Search, DuckDuckGo, Google, Bing hoặc URL tùy chỉnh).
- **Kestrel Shields (Chặn quảng cáo & Tracker 2 tầng)**:
  - **Tầng 1 — Chặn tầng mạng không độ trễ IPC**: Tích hợp [`adblock-rust`](https://github.com/brave/adblock-rust) cùng **EasyList + EasyPrivacy + Fanboy's Annoyances** ngay trong tiến trình con Servo.
  - **Tầng 2 — Quét & Ẩn khung quảng cáo DOM/Cosmetic chủ động**: Tự động loại bỏ các thẻ `<script>`/`<iframe>` quảng cáo và thu gọn các khung chứa quảng cáo rỗng trên các trang báo điện tử (VnExpress/Eclick, Dân Trí/Admicro, Google Ads, Taboola, MGID...).
  - **Bộ đệm nhị phân tối ưu (`adblock_net_v3.blob`)**: Lọc sạch các quy tắc thừa khi biên dịch giúp giảm ~75% dung lượng blob và nạp từ đĩa trong `< 1ms`.
  - **3 cấp độ bảo vệ**: **Tắt (Off)**, **Tiêu chuẩn (Standard)** và **Mạnh (Aggressive)** cùng bộ đếm trực tiếp trên thanh công cụ.
- **Tăng tốc phần cứng GPU tự động**:
  - Tự động nhận diện và kích hoạt GPU rời (NVIDIA PRIME Render Offload + bộ đệm Shader trên đĩa, hoặc AMD/Mesa `mesa_glthread` + `DRI_PRIME`).
- **Hệ thống trang nội bộ (`kestrel://`)**:
  - `kestrel://home` — Trang Dashboard hiển thị trực tiếp số quảng cáo/tracker đã chặn, ước tính dung lượng tiết kiệm, thống kê dấu trang/lịch sử và lưới truy cập nhanh.
  - `kestrel://bookmarks` — Quản lý, tìm kiếm, chuẩn hóa URL và thêm/xóa dấu trang tức thì.
  - `kestrel://history` — Tìm kiếm lịch sử truy cập, tự động gộp các lần truy cập trùng lặp liên tiếp, xóa từng mục hoặc xóa toàn bộ.
  - `kestrel://settings` — Giao diện nút chọn trực quan (Segmented Controls & Cards) áp dụng cài đặt ngay khi nhấp chuột.
  - `kestrel://about` — Thông tin phiên bản, kiến trúc và giấy phép.
- **Lưu trữ SQLite WAL cục bộ**:
  - Toàn bộ dữ liệu được lưu trong `~/.local/share/kestrel-browser/kestrel.sqlite` với chế độ `WAL` tốc độ cao, không đồng bộ đám mây và không theo dõi người dùng.

---

### Tổ hợp phím tắt

| Phím tắt | Chức năng |
|---|---|
| `Ctrl + T` | Mở tab mới và chọn nhanh thanh địa chỉ |
| `Ctrl + W` / `Chuột giữa` | Đóng tab hiện tại |
| `Ctrl + L` / `Alt + D` | Chọn nhanh thanh địa chỉ Omnibox |
| `/` *(trên `kestrel://home`)* | Nhảy nhanh vào ô tìm kiếm trang chủ |
| `Ctrl + R` / `F5` | Tải lại trang (hoặc làm mới dữ liệu trang `kestrel://`) |
| `Alt + Left` | Quay lại trang trước (Back) |
| `Alt + Right` | Tiến tới trang sau (Forward) |
| `Alt + Home` | Về trang chủ (Homepage) |
| `F11` | Bật / tắt chế độ toàn màn hình (Fullscreen) |
| `Ctrl + D` | Lưu hoặc bỏ lưu dấu trang (Bookmark) cho trang hiện tại |
| `Ctrl + B` | Mở trang Quản lý Bookmark (`kestrel://bookmarks`) |
| `Ctrl + H` | Mở trang Lịch sử duyệt web (`kestrel://history`) |
| `Ctrl + ,` | Mở trang Cài đặt (`kestrel://settings`) |
| `Ctrl + Tab` / `Ctrl + PageDown` | Chuyển sang tab kế tiếp |
| `Ctrl + Shift + Tab` / `Ctrl + PageUp` | Quay lại tab liền trước |
| `Ctrl + 1` … `Ctrl + 8` | Chuyển nhanh tới tab số 1 … 8 |
| `Ctrl + 9` | Chuyển tới tab cuối cùng |
| `Ctrl + Q` | Thoát trình duyệt an toàn |

---

### Kiến trúc hệ thống

Kestrel sử dụng bản fork của [`nacho/servo-gtk`](https://github.com/nacho/servo-gtk) được duy trì tại [`LocShadowVN/servo-gtk`](https://github.com/LocShadowVN/servo-gtk). Bản fork tích hợp nhân **Servo 0.6** và mở rộng hook `WebViewDelegate::load_web_resource` bên trong tiến trình con Servo runner.

Nhờ nạp trực tiếp `adblock_net_v3.blob` vào tiến trình con Servo runner và kết hợp tầng quét DOM trong `UserContentManager`, mỗi request tài nguyên phụ đều được kiểm tra bằng lời gọi hàm trực tiếp trong bộ nhớ mà không tốn chi phí truyền thông liên tiến trình (IPC).

```text
┌──────────────────────────────────────────────────────────────────────┐
│ Tiến trình GTK4 chính (kestrel)                                      │
│  ├── Giao diện Chrome (TabBar, Toolbar, Omnibox, Shields, Popovers)  │
│  ├── Lưu trữ SQLite WAL (kestrel.sqlite: History, Bookmarks, Config) │
│  └── Bộ biên dịch & Lưu đệm Adblock nguyên tử (adblock_net_v3.blob)  │
└───────────────┬──────────────────────────────────────▲───────────────┘
                │ IPC (Lệnh điều khiển, Blob, Script)  │ Sự kiện (URI, Title, Số lượng chặn)
┌───────────────▼──────────────────────────────────────┴───────────────┐
│ Tiến trình con Servo Runner (servo-gtk · Servo 0.6)                  │
│  ├── Servo Layout (CSS Grid), WebRender GPU & SpiderMonkey JS        │
│  ├── WebViewDelegate::load_web_resource ──► adblock-rust Engine      │
│  └── UserContentManager (Polyfills v5 & Bộ quét quảng cáo DOM)       │
└──────────────────────────────────────────────────────────────────────┘
```

#### Cấu trúc thư mục (Repository Structure)

```text
kestrel-browser/
├── shared/                     # Mô hình dữ liệu dùng chung (HistoryRecord, BookmarkRecord, AppConfig)
├── src-servo/                  # Crate thực thi chính của trình duyệt
│   ├── build.rs                # Tải và lưu đệm danh sách bộ lọc quảng cáo lúc build
│   └── src/
│       ├── main.rs             # Điểm vào ứng dụng, cấu hình GPU & khởi chạy Servo runner
│       ├── app.rs              # Vòng đời ứng dụng GTK4
│       ├── adblock_rules.rs    # Lọc quy tắc tầng mạng & lưu đệm nhị phân nguyên tử
│       ├── chrome/             # Các thành phần giao diện GTK4 native
│       │   ├── icons.rs        # Hệ thống icon vector SVG đơn sắc kèm bộ nhớ đệm Texture
│       │   ├── style.rs        # CSS toàn cục (Giao diện Dark & Light thích ứng)
│       │   ├── window.rs       # MainWindow, quản lý trạng thái tab, IPC & phím tắt
│       │   ├── tabbar.rs       # Thanh tab cuộn ngang & đồng bộ trang động
│       │   ├── tab_widget.rs   # Widget từng tab hỗ trợ đóng bằng chuột giữa
│       │   ├── toolbar.rs      # Thanh điều hướng, thanh tiến trình & menu chính
│       │   ├── omnibox.rs      # Huy hiệu bảo mật, ô nhập URL, ngôi sao & khiên Shields
│       │   └── adblock_button.rs # Nút Kestrel Shields & popover thống kê trực tiếp
│       ├── compat/
│       │   └── polyfills.rs    # Bộ tương thích Web (Polyfills v5) & Quét quảng cáo DOM
│       ├── engine/
│       │   └── navigation.rs   # Chuẩn hóa URL, nhận diện localhost/IP & định tuyến tìm kiếm
│       ├── internal_pages/     # Các trang nội bộ: kestrel://home, bookmarks, history, settings
│       ├── storage/
│       │   └── database.rs     # Tầng lưu trữ cục bộ SQLite (chế độ WAL)
│       └── util/               # Bộ nhớ đệm cấu hình & ghi log
└── .github/workflows/          # Pipeline CI build & đóng gói phát hành tự động (.deb)
```

---

### Tầng tương thích Web (Polyfills v5)

Dù **Servo 0.6** đã hỗ trợ gốc CSS Grid, `@property`, `touch-action` và `animation-composition`, một số API JavaScript/DOM hiện đại mà các ứng dụng SPA (React, Next.js, Vue, Svelte, Radix UI) yêu cầu vẫn đang được hoàn thiện ở thượng nguồn. Kestrel tự động tiêm bộ polyfill qua `UserContentManager` (chỉ kích hoạt khi phát hiện API chưa tồn tại, tuyệt đối không ghi đè hàm gốc của Servo):

- **Quét & Thu gọn Quảng cáo (Active DOM Scrubber)**: Tự động ẩn các vùng chứa quảng cáo (Google Ads, Eclick, Admicro, Taboola...) và cập nhật trực tiếp bộ đếm Kestrel Shields.
- **Observers & Viewport**: `ResizeObserver`, `IntersectionObserver` (đã khử trùng lặp, giúp hiển thị ảnh lazy-load mượt mà không gây nghẽn Layout), `PerformanceObserver`, `window.visualViewport`.
- **Sao chép dữ liệu sâu**: `structuredClone()` hoàn chỉnh, hỗ trợ `Date`, `RegExp`, `Map`, `Set`, `ArrayBuffer`, `DataView`, `TypedArray` và xử lý tham chiếu vòng qua `WeakMap`.
- **Hoạt ảnh & DOM**: `Element.prototype.animate()`, `getAnimations()`, `checkVisibility()`, `CSS.supports()`, `window.matchMedia`.
- **Chuẩn JS ES2022–ES2024**:
  - Sao chép mảng không đột biến: `Array.prototype.toSorted()`, `toReversed()`, `toSpliced()`, `with()`.
  - Gom nhóm & Chỉ mục: `Object.hasOwn()`, `Object.fromEntries()`, `Object.groupBy()`, `Map.groupBy()`, `Array/String/TypedArray.prototype.at()`, `findLast()`, `findLastIndex()`.
  - Phép toán tập hợp ES2024: `Set.prototype.union()`, `intersection()`, `difference()`.
  - Bất đồng bộ & Tiện ích: `Promise.withResolvers()`, `Promise.allSettled()`, `Promise.any()`, `AggregateError`, `queueMicrotask()`, `requestIdleCallback()`, `URL.canParse()`, `crypto.randomUUID()`.

---

### Yêu cầu hệ thống

- **Hệ điều hành:** Linux `x86_64` (Ubuntu 22.04+, Debian 12+, Fedora 38+, Arch Linux)
- **GPU:** Hỗ trợ phần cứng OpenGL 3.3+ hoặc Vulkan (Intel, AMD Mesa hoặc NVIDIA rời)
- **GTK4:** `libgtk-4-1` (`>= 4.12`)
- **RAM:** Tối thiểu 4 GB khi sử dụng; khuyến nghị 8 GB trở lên khi biên dịch từ mã nguồn

---

### Cài đặt & Build từ Source

#### Cách 1: Cài đặt từ gói `.deb` (Ubuntu / Debian)

Tải file `.deb` mới nhất tại mục [GitHub Releases](https://github.com/LocShadowVN/kestrel-browser/releases) và cài đặt:

```bash
sudo apt install ./kestrel_*.deb
kestrel
```

#### Cách 2: Tự biên dịch từ mã nguồn

1. **Cài đặt các gói phụ thuộc hệ thống (Ubuntu / Debian):**

```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential cmake ninja-build python3 python3-pip \
  pkg-config curl wget file clang llvm lld \
  libssl-dev libgtk-4-dev libepoxy-dev \
  libx11-dev libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libfontconfig1-dev libfreetype6-dev libharfbuzz-dev \
  libgl1-mesa-dev libegl1-mesa-dev libgles2-mesa-dev \
  libgbm-dev libwayland-dev libxkbcommon-dev \
  libsqlite3-dev protobuf-compiler
```

2. **Sao chép mã nguồn và biên dịch:**

```bash
git clone https://github.com/LocShadowVN/kestrel-browser.git
cd kestrel-browser
cargo build --release
```

File thực thi sẽ được tạo tại `target/release/kestrel`. Danh sách bộ lọc quảng cáo (EasyList, EasyPrivacy, Fanboy's Annoyances) được tự động tải về ở lần build đầu tiên và lưu đệm cho các lần build tiếp theo.

3. **Chạy kiểm thử đơn vị (Unit tests):**

```bash
cargo test --workspace
```

> **Biến môi trường khi Build:**
> - `KESTREL_REQUIRE_FILTERS=1`: Dừng build ngay nếu việc tải danh sách bộ lọc EasyList/EasyPrivacy/Fanboy thất bại (dùng trong pipeline phát hành chính thức).
> - `KESTREL_REFRESH_FILTERS=1`: Buộc tải lại danh sách bộ lọc mới nhất ngay cả khi đã có sẵn trong bộ nhớ đệm `$OUT_DIR/kestrel_filters/`.

---

### Lộ trình phát triển (Roadmap)

| Giai đoạn | Nội dung | Trạng thái |
|---|---|---|
| **Phase 0** | Chuẩn hóa CI build & cố định phụ thuộc | Hoàn thành |
| **Phase 1** | Trình duyệt GTK4 cơ bản, tab, SQLite WAL & các trang `kestrel://` | Hoàn thành |
| **Phase 1.5** | Tầng tương thích Web (JS/DOM Polyfills v5) & nâng cấp Servo 0.6 | Hoàn thành |
| **Phase 2** | Chặn quảng cáo 2 tầng (`adblock-rust` + DOM Scrubber) & Kestrel Shields | Hoàn thành |
| **Phase 3** | Khôi phục phiên (Session restore), Tìm trong trang (`Ctrl+F`), Phóng to/Thu nhỏ | Kế hoạch |
| **Phase 4** | Đa phương tiện: Video MSE, WebCodecs, WebRTC | Đợi Servo upstream |
| **Phase 5** | Đóng gói: `.deb` tự động, Flatpak, Arch AUR | Đã có `.deb` |

---

### Giới hạn đã biết

Do nhân Servo đang trong quá trình phát triển tích cực, một số tính năng đa phương tiện phức tạp chưa được hỗ trợ ở tầng engine:

| Tính năng | Nguyên nhân kỹ thuật |
|---|---|
| Xem video YouTube chất lượng cao | Servo chưa hoàn thiện MSE (Media Source Extensions) |
| Netflix, Spotify Web | Chưa hỗ trợ DRM (Widevine CDM / EME) |
| Google Meet, Microsoft Teams | Chưa có đầy đủ ngăn xếp WebCodecs & WebRTC |
| Tiện ích mở rộng Chrome/Firefox | Servo chưa hỗ trợ WebExtensions API |

---

### Đóng góp

Mọi đóng góp, báo lỗi và Pull Request đều được hoan nghênh! Các hạng mục ưu tiên bao gồm:
- Kiểm thử trên nhiều bản phân phối Linux, môi trường Wayland/X11 và cấu hình GPU lai (Hybrid GPU).
- Mở rộng các polyfill đạt chuẩn và quy tắc ẩn quảng cáo trong `src-servo/src/compat/polyfills.rs`.
- Phát triển các tính năng thuộc Giai đoạn 3 (khôi phục phiên, trình quản lý tải xuống, tìm kiếm trong trang).

---

### Ghi nhận & Giấy phép

- **Servo Engine** được phát triển bởi [Igalia](https://www.igalia.com/) và cộng đồng Servo thuộc [Linux Foundation Europe](https://linuxfoundation.eu/).
- **[`servo-gtk`](https://github.com/nacho/servo-gtk)** thượng nguồn được khởi tạo và duy trì bởi [Ignacio Casal Quinteiro (nacho)](https://github.com/nacho).
- **[`adblock-rust`](https://github.com/brave/adblock-rust)** được phát triển bởi [Brave Software](https://brave.com/).

Phát hành dưới giấy phép **GNU General Public License v3.0**. Xem chi tiết tại [LICENSE](LICENSE).
