<div align="center">

# Kestrel Browser

**A lightweight, pure-Rust, zero-telemetry web browser for Linux — built on the Servo 0.6 engine.**

[![CI Build](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml)
[![Release](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/release.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/releases)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Platform: Linux x86_64](https://img.shields.io/badge/platform-Linux%20x86__64-lightgrey.svg?style=flat-square)](#system-requirements)
[![Engine: Servo 0.6](https://img.shields.io/badge/engine-Servo%200.6-orange.svg?style=flat-square)](https://servo.org/)
[![Adblock: Brave adblock-rust](https://img.shields.io/badge/shields-adblock--rust-black.svg?style=flat-square)](https://github.com/brave/adblock-rust)

**English** · [**Tiếng Việt**](README-VN.md)

</div>

---

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
