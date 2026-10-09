//! Polyfill JavaScript cho Servo.
//!
//! Mỗi polyfill dùng feature-detection: chỉ định nghĩa nếu API gốc chưa có.
//! Danh sách tập trung vào các API ES2021-ES2024 mà Servo chưa implement tại
//! revision hiện tại, và các API được framework hiện đại (React, Vue, Svelte)
//! sử dụng rộng rãi.
//!
//! KHÔNG thêm polyfill cho `structuredClone` vì implementation bằng JSON làm
//! mất Date, Map, Set, RegExp và không xử lý circular reference — nguy hiểm
//! hơn là để trang tự xử lý.
//!
//! KHÔNG thêm polyfill cho `ResizeObserver` / `IntersectionObserver` vì chúng
//! cần layout engine của trình duyệt để hoạt động đúng; polyfill sai semantics
//! gây hại nhiều hơn lợi.

/// Script polyfill, được inject vào mọi trang qua `UserContentManager`.
pub const POLYFILL_SCRIPT: &str = r#"
(function () {
    'use strict';

    // ---- Promise.withResolvers() — ES2024 ----
    if (typeof Promise.withResolvers !== 'function') {
        Object.defineProperty(Promise, 'withResolvers', {
            value: function () {
                let resolve, reject;
                const promise = new Promise(function (res, rej) {
                    resolve = res;
                    reject = rej;
                });
                return { promise: promise, resolve: resolve, reject: reject };
            },
            writable: true,
            configurable: true,
        });
    }

    // ---- Object.hasOwn() — ES2022 ----
    if (typeof Object.hasOwn !== 'function') {
        Object.defineProperty(Object, 'hasOwn', {
            value: function (obj, prop) {
                if (obj === null || obj === undefined) {
                    throw new TypeError('Cannot convert undefined or null to object');
                }
                return Object.prototype.hasOwnProperty.call(Object(obj), prop);
            },
            writable: true,
            configurable: true,
        });
    }

    // ---- Array.prototype.at() — ES2022 ----
    if (typeof Array.prototype.at !== 'function') {
        Object.defineProperty(Array.prototype, 'at', {
            value: function (n) {
                var len = this.length >>> 0;
                var k = Math.trunc(n) || 0;
                if (k < 0) k += len;
                if (k < 0 || k >= len) return undefined;
                return this[k];
            },
            writable: true,
            configurable: true,
        });
    }

    // ---- Array.prototype.findLast() — ES2023 ----
    if (typeof Array.prototype.findLast !== 'function') {
        Object.defineProperty(Array.prototype, 'findLast', {
            value: function (predicate, thisArg) {
                if (typeof predicate !== 'function') {
                    throw new TypeError('predicate must be a function');
                }
                for (var i = this.length - 1; i >= 0; i--) {
                    if (predicate.call(thisArg, this[i], i, this)) return this[i];
                }
                return undefined;
            },
            writable: true,
            configurable: true,
        });
    }

    // ---- Array.prototype.findLastIndex() — ES2023 ----
    if (typeof Array.prototype.findLastIndex !== 'function') {
        Object.defineProperty(Array.prototype, 'findLastIndex', {
            value: function (predicate, thisArg) {
                if (typeof predicate !== 'function') {
                    throw new TypeError('predicate must be a function');
                }
                for (var i = this.length - 1; i >= 0; i--) {
                    if (predicate.call(thisArg, this[i], i, this)) return i;
                }
                return -1;
            },
            writable: true,
            configurable: true,
        });
    }

    // ---- String.prototype.replaceAll() — ES2021 ----
    if (typeof String.prototype.replaceAll !== 'function') {
        Object.defineProperty(String.prototype, 'replaceAll', {
            value: function (search, replacement) {
                var str = String(this);
                if (search instanceof RegExp) {
                    if (!search.global) {
                        throw new TypeError('replaceAll requires a global RegExp');
                    }
                    return str.replace(search, replacement);
                }
                var escaped = String(search).replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
                return str.replace(new RegExp(escaped, 'g'), replacement);
            },
            writable: true,
            configurable: true,
        });
    }

    // ---- crypto.randomUUID() — dùng getRandomValues làm nguồn ----
    if (
        typeof crypto !== 'undefined' &&
        typeof crypto.randomUUID !== 'function' &&
        typeof crypto.getRandomValues === 'function'
    ) {
        Object.defineProperty(crypto, 'randomUUID', {
            value: function () {
                var bytes = new Uint8Array(16);
                crypto.getRandomValues(bytes);
                bytes[6] = (bytes[6] & 0x0f) | 0x40;
                bytes[8] = (bytes[8] & 0x3f) | 0x80;
                var hex = [];
                for (var i = 0; i < 16; i++) {
                    hex.push((bytes[i] + 0x100).toString(16).slice(1));
                }
                return (
                    hex.slice(0, 4).join('') + '-' +
                    hex.slice(4, 6).join('') + '-' +
                    hex.slice(6, 8).join('') + '-' +
                    hex.slice(8, 10).join('') + '-' +
                    hex.slice(10, 16).join('')
                );
            },
            writable: true,
            configurable: true,
        });
    }
})();
"#;
