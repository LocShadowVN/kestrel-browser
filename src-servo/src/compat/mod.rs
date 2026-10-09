//! Module tương thích web — inject polyfill JS vào Servo engine.
//!
//! Servo còn thiếu nhiều API JavaScript hiện đại. Module này cung cấp một
//! bộ polyfill nhỏ, được inject qua `UserContentManager` của servo-gtk.
//! Cách tiếp cận: chỉ định nghĩa API nếu nó chưa tồn tại (feature detection),
//! nên không ghi đè implementation native khi Servo đã hỗ trợ.

pub mod polyfills;

pub use polyfills::POLYFILL_SCRIPT;
