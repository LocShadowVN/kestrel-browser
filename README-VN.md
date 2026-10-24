<div align="center">

# Kestrel Browser

**Trình duyệt web nhẹ, thuần Rust, không thu thập dữ liệu dành cho Linux — xây dựng trên nhân Servo 0.6.**

[![CI Build](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/ci.yml)
[![Release](https://github.com/LocShadowVN/kestrel-browser/actions/workflows/release.yml/badge.svg)](https://github.com/LocShadowVN/kestrel-browser/releases)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Platform: Linux x86_64](https://img.shields.io/badge/platform-Linux%20x86__64-lightgrey.svg?style=flat-square)](#yêu-cầu-hệ-thống)
[![Engine: Servo 0.6](https://img.shields.io/badge/engine-Servo%200.6-orange.svg?style=flat-square)](https://servo.org/)
[![Adblock: Brave adblock-rust](https://img.shields.io/badge/shields-adblock--rust-black.svg?style=flat-square)](https://github.com/brave/adblock-rust)

[**English**](README-EN.md) · **Tiếng Việt**

</div>

---

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

Được thiết kế hướng tới tốc độ, mức tiêu thụ RAM thấp và quyền riêng tư cục bộ tuyệt đối, Kestrel hoạt động hoàn toàn không có dịch vụ đám mây chạy ngầm và tích hợp sẵn hệ thống chặn quảng cáo & trình theo dõi 2 tầng.

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
