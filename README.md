# Kestrel Browser

**A lightweight, pure-Rust, zero-telemetry web browser for Linux — built on the Servo engine.**

[![CI Build](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml)
[![Release](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/release.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/releases)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Platform: Linux x86_64](https://img.shields.io/badge/platform-Linux%20x86__64-lightgrey.svg?style=flat-square)](#system-requirements)
[![Engine: Servo](https://img.shields.io/badge/engine-Servo-orange.svg?style=flat-square)](https://servo.org/)
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
- [Web Compatibility Layer](#web-compatibility-layer)
- [System Requirements](#system-requirements)
- [Installation & Building from Source](#installation--building-from-source)
- [Roadmap](#roadmap)
- [Known Limitations](#known-limitations)
- [Previous Generation — Vibird](#previous-generation--vibird)
- [Contributing](#contributing)
- [Credits & License](#credits--license)

---

### Overview

**Kestrel** is an independent, privacy-first Linux web browser written entirely in **Rust**. It pairs a minimalist, monochrome **GTK4** native interface with the next-generation **[Servo](https://servo.org/)** layout engine and **[Brave's `adblock-rust`](https://github.com/brave/adblock-rust)** network filtering engine.

Kestrel is currently in **Beta**: it builds, runs, and renders real-world websites on Linux with tabs, bookmarks, history, customizable settings, keyboard navigation, and zero-IPC network ad blocking.

---

### Key Features

- **Minimalist GTK4 Native UI**:
  - Clean monochrome vector SVG icon system across all toolbars, tabs, popovers, and internal pages.
  - Scrollable floating-pill tab strip with middle-click and `Ctrl+W` tab closing.
  - Instant **Dark / Light theme** switching and **Bilingual UI** (English & Vietnamese).
  - Smooth 2px page-loading progress indicator.
- **Composite Omnibox**:
  - Real-time connection security badge (`HTTPS` lock, `HTTP` warning, `Kestrel` internal scheme indicator).
  - Inline **Bookmark Star (`Ctrl+D`)** that highlights when the current page is saved and toggles bookmarks in one click.
  - Smart URL/domain normalization and customizable search engine templates (Brave Search, DuckDuckGo, Google, Bing, or custom URL).
- **Kestrel Shields (Network-Level Ad & Tracker Blocking)**:
  - Powered by [`adblock-rust`](https://github.com/brave/adblock-rust) with **EasyList**, **EasyPrivacy**, and **Fanboy's Annoyances**.
  - Three protection modes: **Off**, **Standard**, and **Aggressive**.
  - Live per-tab and session-total blocked request counter directly in the toolbar popover.
  - Pre-compiled binary filter cache (`adblock.blob`) for sub-200ms engine startup after the first run.
- **Built-in Internal Pages**:
  - `kestrel://home` — Minimalist new tab dashboard with live Shields/Bookmarks/History statistics and quick navigation cards.
  - `kestrel://bookmarks` — Searchable bookmark manager with inline creation and one-click removal.
  - `kestrel://history` — Searchable browsing history with individual item deletion and full history clearing.
  - `kestrel://settings` — Live configuration for homepage, search engine presets, language, dark/light theme, and Shields level.
  - `kestrel://about` — Engine and license overview.
- **Local SQLite Storage**:
  - Zero cloud sync, zero telemetry. All settings, bookmarks, and history are stored locally in `~/.local/share/kestrel-browser/kestrel.sqlite`.

---

### Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl + T` | Open a new tab |
| `Ctrl + W` / `Middle-Click` | Close the current tab |
| `Ctrl + L` / `Alt + D` | Focus and select the Omnibox address bar |
| `Ctrl + R` / `F5` | Reload the current page (or refresh internal `kestrel://` data) |
| `Alt + Left` | Navigate back |
| `Alt + Right` | Navigate forward |
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

Kestrel uses a downstream fork of [`nacho/servo-gtk`](https://github.com/nacho/servo-gtk) maintained at [`LocShadowVN/servo-gtk`](https://github.com/LocShadowVN/servo-gtk). The fork exposes a `WebViewDelegate::load_web_resource` hook inside the Servo runner subprocess.

Because the `adblock-rust` engine is deserialized directly inside the runner subprocess from a cached binary blob (`~/.local/share/kestrel-browser/adblock.blob`), every subresource request is inspected via a direct in-process function call rather than an expensive IPC round-trip.

```text
┌────────────────────────────────────────────────────────────────────┐
│ Main GTK4 Process (kestrel)                                        │
│  ├── Chrome UI (TabBar, Toolbar, Omnibox, Kestrel Shields, Menus)  │
│  ├── SQLite Storage (kestrel.sqlite: History, Bookmarks, Settings) │
│  └── Adblock Blob Builder & Cache (adblock.blob)                   │
└───────────────┬────────────────────────────────────▲───────────────┘
                │ IPC (Commands, Blob, UserScripts)  │ Events (URI, Title, Blocked Count)
┌───────────────▼────────────────────────────────────┴───────────────┐
│ Servo Runner Subprocess (servo-gtk)                                │
│  ├── Servo Layout & SpiderMonkey JS Engine                         │
│  ├── WebViewDelegate::load_web_resource ──► adblock-rust Engine    │
│  └── Injected Web Compat Polyfills (UserContentManager)            │
└────────────────────────────────────────────────────────────────────┘
```

#### Repository Structure

```text
kestrel-browser/
├── shared/                     # Shared data models (HistoryRecord, BookmarkRecord, AppConfig)
├── src-servo/                  # Main browser binary crate
│   ├── build.rs                # Downloads & embeds filter lists at build time
│   └── src/
│       ├── main.rs             # Entry point & Servo runner subprocess bootstrap
│       ├── app.rs              # GTK4 Application lifecycle
│       ├── adblock_rules.rs    # Filter compilation & binary blob caching
│       ├── chrome/             # Native GTK4 UI components
│       │   ├── icons.rs        # Monochrome inline vector SVG icon system
│       │   ├── style.rs        # Global CSS (Dark & Light themes)
│       │   ├── window.rs       # MainWindow, tab state machine & shortcuts
│       │   ├── tabbar.rs       # Scrollable tab strip & dynamic page tracking
│       │   ├── tab_widget.rs   # Individual tab widget with middle-click close
│       │   ├── toolbar.rs      # Navigation bar, progress bar & hamburger menu
│       │   ├── omnibox.rs      # Security badge, URL entry & bookmark star
│       │   └── adblock_button.rs # Kestrel Shields button & live counter popover
│       ├── compat/
│       │   └── polyfills.rs    # Feature-detected JS/DOM Web Compatibility suite
│       ├── engine/
│       │   └── navigation.rs   # Omnibox URL normalization & search routing
│       ├── internal_pages/     # kestrel://home, bookmarks, history, settings
│       ├── storage/
│       │   └── database.rs     # SQLite persistence layer
│       └── util/               # Config caching & logging
└── .github/workflows/          # CI build & automated .deb release pipelines
```

---

### Web Compatibility Layer

Servo is rapidly evolving, so some modern JavaScript and DOM APIs expected by production SPAs (React, Next.js, Vue, Svelte, Radix UI) are not yet natively exposed. Kestrel injects a strictly **feature-detected** (`typeof ... === 'undefined'`) compatibility layer via `UserContentManager` that never overrides native Servo implementations:

- **Layout & Viewport Observers**: `ResizeObserver`, `IntersectionObserver` (enables lazy-loaded images and prevents SPA mount crashes), `PerformanceObserver`, `window.visualViewport`.
- **Deep Cloning**: Full `structuredClone()` supporting `Date`, `RegExp`, `Map`, `Set`, `ArrayBuffer`, `TypedArray`, and cyclic references via `WeakMap`.
- **Web Animations & DOM**: `Element.prototype.animate()`, `getAnimations()`, `checkVisibility()`, `scrollIntoViewIfNeeded()`, `CSS.supports()`, `CSS.escape()`, dual-listener `window.matchMedia`.
- **ES2022–ES2024 Built-ins**:
  - Change-Array-by-Copy: `Array.prototype.toSorted()`, `toReversed()`, `toSpliced()`, `with()`.
  - Grouping & Indexing: `Object.groupBy()`, `Map.groupBy()`, `Array/String/TypedArray.prototype.at()`, `findLast()`, `findLastIndex()`.
  - ES2024 Set Operations: `union()`, `intersection()`, `difference()`, `symmetricDifference()`, `isSubsetOf()`, `isSupersetOf()`, `isDisjointFrom()`.
  - Async & Messaging: `Promise.withResolvers()`, `Promise.allSettled()`, `Promise.any()`, `AggregateError`, `queueMicrotask()`, `BroadcastChannel`, `requestIdleCallback()`.
- **WHATWG Web APIs**: `URL.canParse()`, `AbortSignal.timeout()`, `AbortSignal.any()`, `crypto.randomUUID()`, `navigator.clipboard`, `navigator.sendBeacon()`.

---

### System Requirements

- **OS:** Linux `x86_64` (Ubuntu 22.04+, Debian 12+, Fedora 38+, Arch Linux)
- **GPU:** Vulkan or OpenGL 3.3+
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
| **Phase 1** | Core GTK4 browser, tabs, SQLite storage & internal pages | Completed |
| **Phase 1.5** | Web Compatibility JS/DOM Polyfill Suite (v3) | Completed |
| **Phase 2** | In-process network ad blocking via `adblock-rust` & Shields UI | Completed |
| **Phase 3** | Privacy & UX: Session restore, Find in Page, Zoom, UA spoofing | Planned |
| **Phase 4** | Rich Media: MSE video streaming, WebCodecs, WebRTC | Upstream Servo |
| **Phase 5** | Packaging: `.deb` automated release, Flatpak, AUR | `.deb` Done / Flatpak Planned |

---

### Known Limitations

Because Servo is under active development, certain web platform features are not yet available upstream:

| Feature | Technical Reason |
|---|---|
| YouTube HD streaming | Servo lacks complete MSE (Media Source Extensions) |
| Netflix, Spotify Web | No Widevine CDM / EME DRM support |
| Google Meet, Microsoft Teams | Incomplete WebRTC & WebCodecs stack |
| Chrome / Firefox Extensions | Servo does not yet implement the WebExtensions API |

---

### Previous Generation — Vibird

If you need a full-featured daily-driver browser today (4-layer adblocking, password vault, download manager, session restore), check out **[LocShadowVN/VibirdBrowser](https://github.com/LocShadowVN/VibirdBrowser)** — Kestrel's WebKitGTK-based predecessor.

---

### Contributing

Contributions, bug reports, and pull requests are welcome! High-impact areas include:
- Testing on diverse Linux distributions, Wayland/X11 compositors, and GPU drivers.
- Expanding safe, spec-compliant shims in `src-servo/src/compat/polyfills.rs`.
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
- [Tầng tương thích Web (Polyfills v3)](#tầng-tương-thích-web-polyfills-v3)
- [Yêu cầu hệ thống](#yêu-cầu-hệ-thống)
- [Cài đặt & Build từ Source](#cài-đặt--build-từ-source)
- [Lộ trình phát triển (Roadmap)](#lộ-trình-phát-triển-roadmap)
- [Giới hạn đã biết](#giới-hạn-đã-biết)
- [Phiên bản tiền nhiệm — Vibird](#phiên-bản-tiền-nhiệm--vibird)
- [Đóng góp & Giấy phép](#đóng-góp--giấy-phép)

---

### Tổng quan

**Kestrel** là trình duyệt web nhẹ, thuần **Rust**, không thu thập dữ liệu (zero-telemetry) dành cho Linux. Kestrel kết hợp giao diện **GTK4** tối giản, đơn sắc với nhân trình duyệt thế hệ mới **[Servo](https://servo.org/)** và bộ máy chặn quảng cáo **[`adblock-rust`](https://github.com/brave/adblock-rust)** của Brave.

Trạng thái hiện tại: **Beta** — biên dịch ổn định, hiển thị các trang web thực tế trên Linux với đầy đủ quản lý tab, lịch sử, dấu trang, cài đặt trực quan, phím tắt đầy đủ và chặn quảng cáo ở tầng mạng.

---

### Tính năng nổi bật

- **Giao diện GTK4 Tối giản & Đơn sắc (Monochrome SVG)**:
  - Toàn bộ hệ thống biểu tượng được vẽ bằng vector SVG nét mảnh đơn sắc đồng bộ từ thanh công cụ xuống các trang nội bộ.
  - Thanh tab cuộn ngang mượt mà, hỗ trợ đóng tab bằng **chuột giữa (Middle-click)** hoặc `Ctrl+W`.
  - Chuyển đổi tức thì giữa **Chế độ Tối (Dark Mode)** và **Chế độ Sáng (Light Mode)**, hỗ trợ song ngữ **Tiếng Anh & Tiếng Việt**.
  - Thanh tiến trình tải trang (Progress Bar 2px) mượt mà.
- **Thanh địa chỉ Omnibox hợp nhất**:
  - Biểu tượng trạng thái bảo mật trực quan (Khóa `HTTPS`, cảnh báo `HTTP`, huy hiệu trang nội bộ `Kestrel`).
  - Tích hợp **Nút ngôi sao Bookmark (`Ctrl+D`)** ngay trong thanh địa chỉ: tự động sáng lên khi trang đã được lưu và cho phép thêm/xóa dấu trang chỉ với 1 cú nhấp.
  - Tự động nhận diện tên miền, IP nội bộ và hỗ trợ chọn nhanh công cụ tìm kiếm (Brave Search, DuckDuckGo, Google, Bing hoặc URL tùy chỉnh).
- **Kestrel Shields (Chặn quảng cáo & Tracker tầng mạng)**:
  - Sức mạnh từ [`adblock-rust`](https://github.com/brave/adblock-rust) kết hợp bộ quy tắc **EasyList + EasyPrivacy + Fanboy's Annoyances**.
  - 3 cấp độ bảo vệ: **Tắt (Off)**, **Tiêu chuẩn (Standard)** và **Mạnh (Aggressive)**.
  - Hiển thị **bộ đếm số quảng cáo/tracker đã chặn** trực tiếp trên thanh công cụ cho từng tab và toàn phiên duyệt web.
  - Tự động biên dịch và lưu đệm bộ lọc ra file nhị phân (`adblock.blob`), giúp các lần khởi động sau chỉ mất ~50–200ms.
- **Hệ thống trang nội bộ (`kestrel://`)**:
  - `kestrel://home` — Trang tab mới tối giản với thống kê Shields/Bookmarks/History và các thẻ truy cập nhanh.
  - `kestrel://bookmarks` — Quản lý, tìm kiếm, thêm mới và xóa dấu trang nhanh.
  - `kestrel://history` — Tìm kiếm lịch sử truy cập, xóa từng mục hoặc xóa toàn bộ lịch sử.
  - `kestrel://settings` — Tùy chỉnh trang chủ, công cụ tìm kiếm, ngôn ngữ, giao diện Sáng/Tối và cấp độ chặn quảng cáo.
  - `kestrel://about` — Thông tin phiên bản và giấy phép.
- **Lưu trữ cục bộ bằng SQLite**:
  - Toàn bộ lịch sử, bookmark và cài đặt được lưu trong `~/.local/share/kestrel-browser/kestrel.sqlite`.

---

### Tổ hợp phím tắt

| Phím tắt | Chức năng |
|---|---|
| `Ctrl + T` | Mở tab mới |
| `Ctrl + W` / `Chuột giữa` | Đóng tab hiện tại |
| `Ctrl + L` / `Alt + D` | Chọn nhanh thanh địa chỉ Omnibox |
| `Ctrl + R` / `F5` | Tải lại trang (hoặc làm mới dữ liệu trang nội bộ `kestrel://`) |
| `Alt + Left` | Quay lại trang trước (Back) |
| `Alt + Right` | Tiến tới trang sau (Forward) |
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

Kestrel sử dụng bản fork của [`nacho/servo-gtk`](https://github.com/nacho/servo-gtk) tại [`LocShadowVN/servo-gtk`](https://github.com/LocShadowVN/servo-gtk). Bản fork bổ sung hook `WebViewDelegate::load_web_resource` để kiểm tra và chặn các request quảng cáo/tracker ngay trước khi Servo gửi đi.

Nhờ nạp trực tiếp `adblock.blob` vào tiến trình con Servo runner, việc kiểm tra mỗi request chỉ là một lời gọi hàm nội bộ thay vì phải đi vòng qua IPC.

---

### Tầng tương thích Web (Polyfills v3)

Để khắc phục việc các trang web hiện đại (React, Next.js, Vue, Svelte, Radix UI) bị lỗi màn hình trắng khi thiếu một số API trên Servo, Kestrel tự động tiêm bộ polyfill thông minh qua `UserContentManager` (chỉ kích hoạt khi phát hiện API chưa tồn tại):

- **Observers & Viewport**: `ResizeObserver`, `IntersectionObserver` (giúp hiển thị ảnh lazy-load và cuộn vô tận), `PerformanceObserver`, `window.visualViewport`.
- **Sao chép dữ liệu sâu**: `structuredClone()` hoàn chỉnh, giữ nguyên `Date`, `RegExp`, `Map`, `Set`, `ArrayBuffer`, `TypedArray` và xử lý an toàn tham chiếu vòng qua `WeakMap`.
- **Hoạt ảnh & DOM**: `Element.prototype.animate()`, `getAnimations()`, `checkVisibility()`, `scrollIntoViewIfNeeded()`, `CSS.supports()`, `CSS.escape()`, chuẩn hóa `window.matchMedia`.
- **Chuẩn JS ES2022–ES2024**: `toSorted()`, `toReversed()`, `toSpliced()`, `with()`, `Object.groupBy()`, `Map.groupBy()`, các phép toán tập hợp trên `Set`, `Promise.withResolvers()`, `Promise.allSettled()`, `Promise.any()`, `BroadcastChannel`, `URL.canParse()`, `AbortSignal.timeout()`, `AbortSignal.any()`.

---

### Yêu cầu hệ thống

- **Hệ điều hành:** Linux `x86_64` (Ubuntu 22.04+, Debian 12+, Fedora 38+, Arch Linux)
- **GPU:** Hỗ trợ Vulkan hoặc OpenGL 3.3+
- **GTK4:** Có sẵn trên hầu hết các bản phân phối Linux hiện đại (`>= 4.12`)
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

```bash
git clone https://github.com/LocShadowVN/kestrel-browser.git
cd kestrel-browser
cargo build --release
```

File thực thi được tạo tại `target/release/kestrel`. Danh sách bộ lọc quảng cáo (EasyList, EasyPrivacy, Fanboy's Annoyances) sẽ tự động được tải về ở lần build đầu tiên và lưu đệm cho các lần build tiếp theo.

---

### Lộ trình phát triển (Roadmap)

| Giai đoạn | Nội dung | Trạng thái |
|---|---|---|
| **Phase 0** | Chuẩn hóa CI build & cố định phụ thuộc | Hoàn thành |
| **Phase 1** | Trình duyệt GTK4 cơ bản, tab, SQLite & các trang `kestrel://` | Hoàn thành |
| **Phase 1.5** | Tầng tương thích Web (JS/DOM Polyfills v3) | Hoàn thành |
| **Phase 2** | Chặn quảng cáo tầng mạng qua `adblock-rust` & Kestrel Shields | Hoàn thành |
| **Phase 3** | Khôi phục phiên (Session restore), Tìm trong trang, Zoom, UA spoofing | Kế hoạch |
| **Phase 4** | Đa phương tiện: Video MSE, WebCodecs, WebRTC | Đợi Servo upstream |
| **Phase 5** | Đóng gói: `.deb` tự động, Flatpak, AUR | Đã có `.deb` |

---

### Giới hạn đã biết

Do nhân Servo đang trong quá trình phát triển tích cực, một số tính năng đa phương tiện phức tạp chưa được hỗ trợ ở tầng engine:

| Tính năng | Nguyên nhân kỹ thuật |
|---|---|
| Xem video YouTube chất lượng cao | Servo chưa hoàn thiện MSE (Media Source Extensions) |
| Netflix, Spotify Web | Chưa hỗ trợ DRM (Widevine CDM) |
| Google Meet, Microsoft Teams | Chưa có đầy đủ WebCodecs & WebRTC |
| Tiện ích mở rộng Chrome/Firefox | Servo chưa hỗ trợ WebExtensions API |

---

### Phiên bản tiền nhiệm — Vibird

Nếu bạn cần một trình duyệt đầy đủ mọi tính năng ngay hôm nay (chặn quảng cáo 4 tầng, kho mật khẩu, quản lý tải xuống, khôi phục phiên), hãy tham khảo **[LocShadowVN/VibirdBrowser](https://github.com/LocShadowVN/VibirdBrowser)** — phiên bản tiền nhiệm sử dụng nhân WebKitGTK.

---

### Đóng góp & Giấy phép

Mọi đóng góp (báo lỗi, kiểm thử trên các cấu hình GPU/Wayland/X11, mở rộng polyfill hoặc dịch thuật) đều được hoan nghênh.

Phát hành dưới giấy phép **GNU General Public License v3.0**. Xem chi tiết tại [LICENSE](LICENSE).
