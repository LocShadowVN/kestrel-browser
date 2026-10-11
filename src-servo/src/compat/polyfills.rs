//! Polyfill JavaScript cho Servo.
//!
//! Mỗi polyfill dùng feature-detection: chỉ định nghĩa nếu API gốc chưa có.
//! Danh sách tập trung vào các API ES2020-ES2024 mà Servo chưa implement.
//!
//! KHÔNG polyfill: `structuredClone` (mất Date/Map/Set/RegExp),
//! `ResizeObserver`/`IntersectionObserver` (cần layout engine),
//! `Element.animate()` (cần Web Animations engine).

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
            writable: true, configurable: true,
        });
    }

    // ---- queueMicrotask() — cần cho framework async ----
    if (typeof queueMicrotask !== 'function') {
        globalThis.queueMicrotask = function (cb) {
            if (typeof cb !== 'function') {
                throw new TypeError('queueMicrotask requires a function');
            }
            Promise.resolve().then(cb).catch(function (e) {
                setTimeout(function () { throw e; }, 0);
            });
        };
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
            writable: true, configurable: true,
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
            writable: true, configurable: true,
        });
    }

    // ---- Array.prototype.flat() — ES2019 ----
    if (typeof Array.prototype.flat !== 'function') {
        Object.defineProperty(Array.prototype, 'flat', {
            value: function (depth) {
                var d = depth === undefined ? 1 : Math.trunc(depth);
                var result = [];
                (function flatten(arr, level) {
                    for (var i = 0; i < arr.length; i++) {
                        if (Array.isArray(arr[i]) && level > 0) {
                            flatten(arr[i], level - 1);
                        } else if (i in arr) {
                            result.push(arr[i]);
                        }
                    }
                })(this, d);
                return result;
            },
            writable: true, configurable: true,
        });
    }

    // ---- Array.prototype.flatMap() — ES2019 ----
    if (typeof Array.prototype.flatMap !== 'function') {
        Object.defineProperty(Array.prototype, 'flatMap', {
            value: function (cb, thisArg) {
                if (typeof cb !== 'function') {
                    throw new TypeError('flatMap requires a function');
                }
                var result = [];
                for (var i = 0; i < this.length; i++) {
                    if (i in this) {
                        var val = cb.call(thisArg, this[i], i, this);
                        if (Array.isArray(val)) {
                            for (var j = 0; j < val.length; j++) result.push(val[j]);
                        } else {
                            result.push(val);
                        }
                    }
                }
                return result;
            },
            writable: true, configurable: true,
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
            writable: true, configurable: true,
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
            writable: true, configurable: true,
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
            writable: true, configurable: true,
        });
    }

    // ---- Object.fromEntries() — ES2019 ----
    if (typeof Object.fromEntries !== 'function') {
        Object.defineProperty(Object, 'fromEntries', {
            value: function (iterable) {
                var obj = {};
                for (var i = 0; i < iterable.length; i++) {
                    obj[iterable[i][0]] = iterable[i][1];
                }
                return obj;
            },
            writable: true, configurable: true,
        });
    }

    // ---- crypto.randomUUID() — fallback qua getRandomValues ----
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
            writable: true, configurable: true,
        });
    }

    // ---- requestIdleCallback() — scheduler API ----
    // Polyfill tối thiểu: dùng setTimeout, timeRemaining() trả về budget cố định.
    if (typeof requestIdleCallback !== 'function') {
        globalThis.requestIdleCallback = function (cb) {
            var start = Date.now();
            var handle = setTimeout(function () {
                cb({
                    didTimeout: false,
                    timeRemaining: function () {
                        return Math.max(0, 50 - (Date.now() - start));
                    },
                });
            }, 1);
            return handle;
        };
        globalThis.cancelIdleCallback = function (handle) {
            clearTimeout(handle);
        };
    }

    console.log('[Kestrel] polyfills loaded');
})();
"#;
