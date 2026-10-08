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
