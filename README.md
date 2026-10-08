# Kestrel Browser

Trình duyệt desktop cho Linux, xây trên **Servo engine** — viết bằng Rust.

[![Status](https://img.shields.io/badge/status-experimental-red.svg?style=flat-square)](#trạng-thái)
[![Build](https://img.shields.io/badge/build-not%20passing-red.svg?style=flat-square)](#trạng-thái)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20x86__64-lightgrey.svg?style=flat-square)](#yêu-cầu)
[![Engine](https://img.shields.io/badge/engine-Servo-orange.svg?style=flat-square)](https://servo.org/)

[Tiếng Việt](#tiếng-việt) · [English](#english)

</div>

---

## ⚠️ TRẠNG THÁI HIỆN TẠI

> **DỰ ÁN CHƯA CHẠY ĐƯỢC.**
>
> Binary chưa compile thành công. Đang trong quá trình giải quyết dependency
> conflicts giữa `servo`, `servo-gtk` và các crate khác.
>
> **Không cài được. Không có release. Không có binary.**
>
> Đây là dự án thử nghiệm cá nhân. Xem [Về dự án](#về-dự-án) để hiểu rõ hơn.

---

<a name="tiếng-việt"></a>
## Tiếng Việt

### Về dự án

Kestrel là dự án cá nhân nhằm xây dựng một trình duyệt desktop cho Linux
sử dụng **Servo engine** — engine trình duyệt thuần Rust do Linux Foundation
Europe bảo trợ và Igalia phát triển chính.

**Mục tiêu dài hạn:** một trình duyệt nhẹ, thuần Rust, chạy được trên máy yếu,
không bundle Chromium, không telemetry.

**Thực tế hiện tại:** dự án đang ở giai đoạn **rất sớm**. Code đã viết, nhưng
chưa build thành công do dependency conflicts. Chưa có gì chạy được để cho
bạn dùng thử.

### Tại sao viết lại?

Phiên bản trước — **Vibird** — dùng WebKitGTK qua Tauri. Vibird chạy được,
có adblock 4 tầng, có vault, có download. Nhưng nó có một vấn đề không fix được:

**WebKitGTK bị bug compositor trên Nvidia + Wayland.** Trên một số cấu hình
GPU, ứng dụng có thể bị OOM kill hoặc văng ra login screen sau vài phút sử
dụng. Đây là bug của WebKitGTK, không phải của Vibird — nó ảnh hưởng đến mọi
ứng dụng dùng WebKitGTK bao gồm GNOME Web, Devhelp và Yelp.

Sau khi cân nhắc nhiều lựa chọn (WebKitGTK, CEF, Blitz, Sciter, Ultralight),
chỉ có Servo đáp ứng các tiêu chí:
- Viết bằng Rust
- Nhẹ, thiết kế cho nhúng
- Có tương lai (được tài trợ bởi Sovereign Tech Fund, phát triển bởi Igalia)

Kestrel là canh bạc dài hạn vào Servo. Đánh đổi được chấp nhận: web
compatibility thấp hơn WebKit (78% WPT vs 96%), nhưng thoát khỏi bug
Nvidia + Wayland.

### Trạng thái phát triển

#### ❌ Chưa hoạt động

| Thành phần | Trạng thái | Ghi chú |
|---|---|---|
| Build | ❌ **Fail** | Dependency conflicts chưa giải quyết xong |
| Binary | ❌ Chưa có | Không có release nào |
| Cài đặt | ❌ Không thể | Chưa có gói cài đặt |
| Chạy được | ❌ Không | Không có gì để chạy |

#### 🔧 Đang phát triển

| Thành phần | Trạng thái | Ghi chú |
|---|---|---|
| Kiến trúc code | ✅ Đã viết | Structure đầy đủ, chưa compile |
| GTK4 UI | ✅ Đã viết | Window, toolbar, omnibox, tabbar |
| Servo integration | ⚠️ Đang fix | Dependency conflicts |
| SQLite storage | ✅ Đã viết | Schema + queries |
| URL normalization | ✅ Đã viết + test | Có unit tests |

#### 📋 Kế hoạch

| Phase | Nội dung | Trạng thái |
|---|---|---|
| Phase 0 | Build thành công | 🔧 Đang làm |
| Phase 1 | Browser cơ bản chạy được | ⏸ Chờ Phase 0 |
| Phase 2 | Adblock (network + cosmetic) | ⏸ Chờ Phase 1 |
| Phase 3 | Privacy (vault, UA spoof) | ⏸ Chờ Phase 2 |
| Phase 4 | Media (video, audio) | ⏸ Chờ Servo upstream |

### Yêu cầu

Nếu dự án build thành công trong tương lai:

- **Hệ điều hành:** Linux x86_64
- **GPU:** Vulkan hoặc OpenGL 3.3+
- **GTK4:** có sẵn trên Ubuntu 22.04+, Fedora 37+, Arch, v.v.
- **RAM:** 4 GB tối thiểu, 8 GB khuyến nghị (cho quá trình build)

### Build từ source

**Cảnh báo:** Hiện tại build sẽ fail. Xem mục [Vấn đề đã biết](#vấn-đề-đã-biết).

```bash
git clone https://github.com/LocShadowVN/kestrel-browser.git
cd kestrel-browser
cargo build --release
```

### Vấn đề đã biết

Dự án đang gặp các vấn đề sau trong quá trình build:

1. **`gl_generator` và `xml-rs` conflict** — `epoxy 0.1` phụ thuộc `gl_generator 0.9`,
   crate này yêu cầu `xml-rs 0.7` đã bị yanked khỏi crates.io. Cần patch qua Git.
2. **`libsqlite3-sys` conflict** — `kestrel` và `servo-storage` yêu cầu các phiên
   bản `rusqlite` khác nhau, gây xung đột native library link.
3. **Servo API chưa ổn định** — crate `servo` và `servo-gtk` đang phát triển tích
   cực, API có thể thay đổi giữa các commit.

Các vấn đề này đang được giải quyết từng bước. Theo dõi tab
[Issues](https://github.com/LocShadowVN/kestrel-browser/issues) để cập nhật.

### Không hỗ trợ

Khi dự án chạy được (trong tương lai), các tính năng sau sẽ **không** hoạt động
do giới hạn của Servo engine:

| Tính năng | Lý do |
|---|---|
| YouTube video | Servo chưa implement MSE (Media Source Extensions) |
| Netflix, Spotify Web | Thiếu DRM (Widevine CDM) |
| Google Meet, Microsoft Teams | Thiếu WebCodecs |
| Extension Chrome/Firefox | Servo không hỗ trợ WebExtension API |
| Google Docs editor | Thiếu một số API DOM phức tạp |

### Phiên bản trước — Vibird

Nếu bạn cần một trình duyệt chạy được ngay bây giờ với đầy đủ tính năng
(adblock 4 tầng, vault, download, session), xem phiên bản cũ **Vibird v2.4.2**
dùng WebKitGTK. Nó có tag `v2.4.2-webkit-final` trong repo.

**Lưu ý:** Vibird có bug Nvidia + Wayland đã nêu ở trên. Trên Intel/AMD iGPU
và X11, nó chạy ổn định.

### Đóng góp

Dự án đang ở giai đoạn rất sớm. Đóng góp hoan nghênh, đặc biệt:

- Fix dependency conflicts (xem mục Vấn đề đã biết)
- Báo cáo lỗi build
- Test trên các cấu hình khác nhau
- Cải thiện documentation

### License

GNU General Public License v3.0. Xem [LICENSE](LICENSE).

---

<a name="english"></a>
## English

### ⚠️ CURRENT STATUS

> **THE PROJECT DOES NOT BUILD.**
>
> The binary does not compile. We are working through dependency conflicts
> between `servo`, `servo-gtk`, and other crates.
>
> **No installation. No release. No binary.**
>
> This is a personal experimental project. See [About](#about) for details.

### About

Kestrel is a personal project to build a desktop browser for Linux using the
**Servo engine** — a pure-Rust browser engine backed by Linux Foundation Europe
and developed primarily by Igalia.

**Long-term goal:** a lightweight, pure-Rust browser that runs on low-end
machines, does not bundle Chromium, and has zero telemetry.

**Current reality:** the project is in a **very early stage**. Code is written,
but does not build due to dependency conflicts. Nothing is available to try.

### Why a rewrite?

The previous version — **Vibird** — used WebKitGTK through Tauri. Vibird
worked, had 4-layer adblock, vault, and downloads. But it had one unfixable
problem:

**WebKitGTK has compositor bugs on Nvidia + Wayland.** On some GPU
configurations, the app could be OOM-killed or dropped to login screen within
minutes. This is a WebKitGTK bug, not Vibird's — it affects every WebKitGTK
app including GNOME Web, Devhelp, and Yelp.

After evaluating alternatives (WebKitGTK, CEF, Blitz, Sciter, Ultralight),
only Servo met the criteria:
- Written in Rust
- Lightweight, embedding-first
- Has a future (Sovereign Tech Fund backed, Igalia-led)

Kestrel is a long-term bet on Servo. Trade-off accepted: lower web
compatibility (78% WPT vs 96%), but escapes the Nvidia + Wayland bugs.

### Development Status

#### ❌ Not Working

| Component | Status | Notes |
|---|---|---|
| Build | ❌ **Failing** | Dependency conflicts unresolved |
| Binary | ❌ None | No releases |
| Installation | ❌ Not possible | No installers |
| Runtime | ❌ N/A | Nothing to run |

#### 🔧 In Development

| Component | Status | Notes |
|---|---|---|
| Architecture | ✅ Written | Full structure, not compiling |
| GTK4 UI | ✅ Written | Window, toolbar, omnibox, tabbar |
| Servo integration | ⚠️ Fixing | Dependency conflicts |
| SQLite storage | ✅ Written | Schema + queries |
| URL normalization | ✅ Written + tested | Unit tests present |

#### 📋 Roadmap

| Phase | Content | Status |
|---|---|---|
| Phase 0 | Successful build | 🔧 In progress |
| Phase 1 | Basic working browser | ⏸ Blocked by Phase 0 |
| Phase 2 | Adblock (network + cosmetic) | ⏸ Blocked by Phase 1 |
| Phase 3 | Privacy (vault, UA spoof) | ⏸ Blocked by Phase 2 |
| Phase 4 | Media (video, audio) | ⏸ Blocked by Servo upstream |

### Requirements

If the project builds successfully in the future:

- **OS:** Linux x86_64
- **GPU:** Vulkan or OpenGL 3.3+
- **GTK4:** available on Ubuntu 22.04+, Fedora 37+, Arch, etc.
- **RAM:** 4 GB minimum, 8 GB recommended for build

### Building from Source

**Warning:** Build currently fails. See [Known Issues](#known-issues).

```bash
git clone https://github.com/LocShadowVN/kestrel-browser.git
cd kestrel-browser
cargo build --release
```

### Known Issues

The project is currently blocked by:

1. **`gl_generator` / `xml-rs` conflict** — `epoxy 0.1` depends on
   `gl_generator 0.9`, which requires `xml-rs 0.7` (yanked from crates.io).
   Requires Git patch.
2. **`libsqlite3-sys` conflict** — `kestrel` and `servo-storage` require
   different `rusqlite` versions, causing native library link conflicts.
3. **Servo API instability** — `servo` and `servo-gtk` crates are actively
   developed; API may change between commits.

These are being resolved incrementally. Track progress in
[Issues](https://github.com/LocShadowVN/kestrel-browser/issues).

### Not Supported

When the project eventually works, the following will **not** work due to
Servo engine limitations:

| Feature | Reason |
|---|---|
| YouTube video | Servo lacks MSE (Media Source Extensions) |
| Netflix, Spotify Web | No DRM (Widevine CDM) |
| Google Meet, Microsoft Teams | No WebCodecs |
| Chrome/Firefox extensions | Servo lacks WebExtension API |
| Google Docs editor | Missing complex DOM APIs |

### Previous Version — Vibird

If you need a working browser now with full features (4-layer adblock, vault,
downloads, session), see the previous **Vibird v2.4.2** using WebKitGTK.
It has tag `v2.4.2-webkit-final`.

**Note:** Vibird has the Nvidia + Wayland bug described above. On Intel/AMD
iGPU with X11, it runs stably.

### Contributing

The project is very early stage. Contributions welcome, especially:

- Fix dependency conflicts (see Known Issues)
- Report build errors
- Test on different configurations
- Improve documentation

### License

GNU General Public License v3.0. See [LICENSE](LICENSE).
