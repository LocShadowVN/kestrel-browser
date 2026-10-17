//! Polyfill JavaScript nhẹ, mượt + Cosmetic Adblock CSS cho Servo Engine.
//! - Không gọi `getBoundingClientRect()` hàng loạt trong Observer để tránh nghẽn IPC Layout.
//! - Tự động bơm bảng CSS ẩn các khung quảng cáo rỗng (Google Ads, Eclick/VnExpress, Admicro, Taboola, MGID...).

pub const POLYFILL_SCRIPT: &str = r#"
(function () {
    'use strict';

    // ---- 1. globalThis ----
    if (typeof globalThis === 'undefined') {
        if (typeof window !== 'undefined') window.globalThis = window;
        else if (typeof self !== 'undefined') self.globalThis = self;
    }

    // ---- 2. Cosmetic Adblock CSS (Ẩn ngay lập tức các khung quảng cáo rỗng trên VnExpress, Dân Trí & quốc tế) ----
    var COSMETIC_ADBLOCK_CSS = [
        /* Google AdSense, GPT & DoubleClick */
        'ins.adsbygoogle, .adsbygoogle, [id^="div-gpt-ad"], [id^="google_ads_iframe"],',
        'iframe[id^="google_ads_"], iframe[src*="doubleclick.net"], iframe[src*="googlesyndication.com"],',
        'iframe[src*="adservice.google."], [data-ad-client], [data-ad-slot], [data-google-query-id],',
        /* VnExpress & FPT Eclick */
        '[id^="sis_"], [class^="sis_"], [id^="eclick"], [class*="eclick"], iframe[src*="eclick.vn"],',
        '.banner-ads, .box-ads, .ads-top, .ads-right, .ads-middle, .ads-bottom, .ads-sticky,',
        '.section_banner_top, .section-banner-top, .banner_top, .wrap-banner-top, .lazy-ads,',
        '[id^="banner_top"], [id^="ads_"], [class*="banner_ads"], .item_box_ads, .box_quangcao,',
        '[data-component="Banner"], [data-component-value*="banner"], .header-banner-ads,',
        /* Admicro / VCCorp (Dân Trí, Kenh14, CafeF, Tuổi Trẻ, Thanh Niên) */
        '[id^="adm-"], [id^="adm_"], [id^="admicro"], [class*="admicro"], .adm-ads, .admzone,',
        '[id^="admzone"], iframe[src*="admicro.vn"], iframe[src*="vcmedia.vn/ads"],',
        '.ads-zone, .qc-box, .box-qc, .banner-qc, .quang-cao, .box-quang-cao,',
        /* Taboola, Outbrain, MGID, Criteo & Generic EasyList Containers */
        '.taboola, .outbrain, [id^="taboola-"], [class*="outbrain"], [id^="mgid"], [class*="mgid"],',
        'iframe[src*="criteo."], .ad-banner, .ad-container, .ad-slot, .ad-wrapper, .ad-placeholder,',
        '.ad-box, .ad-unit, .ad-leaderboard, .ad-billboard, .advertisement, .sponsored-content',
        '{',
        '  display: none !important;',
        '  visibility: hidden !important;',
        '  height: 0 !important;',
        '  min-height: 0 !important;',
        '  max-height: 0 !important;',
        '  margin: 0 !important;',
        '  padding: 0 !important;',
        '  border: none !important;',
        '  overflow: hidden !important;',
        '  pointer-events: none !important;',
        '}'
    ].join('\n');

    function injectCosmeticAdblock() {
        if (typeof document === 'undefined') return;
        if (document.getElementById('kestrel-cosmetic-adblock')) return;
        var target = document.head || document.documentElement;
        if (!target) return;
        var style = document.createElement('style');
        style.id = 'kestrel-cosmetic-adblock';
        style.textContent = COSMETIC_ADBLOCK_CSS;
        target.appendChild(style);
    }

    injectCosmeticAdblock();
    if (typeof document !== 'undefined') {
        document.addEventListener('DOMContentLoaded', injectCosmeticAdblock, { once: true });
    }

    // ---- 3. Promise.withResolvers() — ES2024 ----
    if (typeof Promise !== 'undefined' && typeof Promise.withResolvers !== 'function') {
        Object.defineProperty(Promise, 'withResolvers', {
            value: function () {
                var resolve, reject;
                var promise = new Promise(function (res, rej) {
                    resolve = res;
                    reject = rej;
                });
                return { promise: promise, resolve: resolve, reject: reject };
            },
            writable: true,
            configurable: true
        });
    }

    // ---- 4. Promise.allSettled() & Promise.any() ----
    if (typeof Promise !== 'undefined' && typeof Promise.allSettled !== 'function') {
        Object.defineProperty(Promise, 'allSettled', {
            value: function (iterable) {
                var arr = Array.from(iterable);
                return Promise.all(arr.map(function (item) {
                    return Promise.resolve(item).then(
                        function (value) { return { status: 'fulfilled', value: value }; },
                        function (reason) { return { status: 'rejected', reason: reason }; }
                    );
                }));
            },
            writable: true,
            configurable: true
        });
    }

    if (typeof AggregateError === 'undefined') {
        globalThis.AggregateError = function AggregateError(errors, message) {
            var err = new Error(message || 'All promises were rejected');
            err.name = 'AggregateError';
            err.errors = Array.isArray(errors) ? errors : Array.from(errors || []);
            return err;
        };
        globalThis.AggregateError.prototype = Object.create(Error.prototype);
    }

    if (typeof Promise !== 'undefined' && typeof Promise.any !== 'function') {
        Object.defineProperty(Promise, 'any', {
            value: function (iterable) {
                var arr = Array.from(iterable);
                if (arr.length === 0) {
                    return Promise.reject(new AggregateError([], 'All promises were rejected'));
                }
                return new Promise(function (resolve, reject) {
                    var errors = new Array(arr.length);
                    var rejectedCount = 0;
                    arr.forEach(function (item, idx) {
                        Promise.resolve(item).then(resolve, function (err) {
                            errors[idx] = err;
                            rejectedCount++;
                            if (rejectedCount === arr.length) {
                                reject(new AggregateError(errors, 'All promises were rejected'));
                            }
                        });
                    });
                });
            },
            writable: true,
            configurable: true
        });
    }

    // ---- 5. queueMicrotask() ----
    if (typeof queueMicrotask !== 'function') {
        globalThis.queueMicrotask = function (cb) {
            if (typeof cb !== 'function') throw new TypeError('queueMicrotask requires a function');
            Promise.resolve().then(cb).catch(function (e) {
                setTimeout(function () { throw e; }, 0);
            });
        };
    }

    // ---- 6. Object.hasOwn() & Object.fromEntries() ----
    if (typeof Object.hasOwn !== 'function') {
        Object.defineProperty(Object, 'hasOwn', {
            value: function (obj, prop) {
                if (obj === null || obj === undefined) {
                    throw new TypeError('Cannot convert undefined or null to object');
                }
                return Object.prototype.hasOwnProperty.call(Object(obj), prop);
            },
            writable: true,
            configurable: true
        });
    }

    if (typeof Object.fromEntries !== 'function') {
        Object.defineProperty(Object, 'fromEntries', {
            value: function (iterable) {
                var obj = {};
                for (var item of iterable) {
                    if (item && typeof item === 'object') {
                        obj[item[0]] = item[1];
                    }
                }
                return obj;
            },
            writable: true,
            configurable: true
        });
    }

    // ---- 7. Object.groupBy() & Map.groupBy() — ES2024 ----
    if (typeof Object.groupBy !== 'function') {
        Object.defineProperty(Object, 'groupBy', {
            value: function (items, callbackFn) {
                if (typeof callbackFn !== 'function') throw new TypeError('callbackFn must be a function');
                var obj = Object.create(null);
                var k = 0;
                for (var item of items) {
                    var key = callbackFn(item, k++);
                    if (!(key in obj)) obj[key] = [];
                    obj[key].push(item);
                }
                return obj;
            },
            writable: true,
            configurable: true
        });
    }

    if (typeof Map !== 'undefined' && typeof Map.groupBy !== 'function') {
        Object.defineProperty(Map, 'groupBy', {
            value: function (items, callbackFn) {
                if (typeof callbackFn !== 'function') throw new TypeError('callbackFn must be a function');
                var map = new Map();
                var k = 0;
                for (var item of items) {
                    var key = callbackFn(item, k++);
                    var list = map.get(key);
                    if (!list) {
                        list = [];
                        map.set(key, list);
                    }
                    list.push(item);
                }
                return map;
            },
            writable: true,
            configurable: true
        });
    }

    // ---- 8. Array/String.prototype.at — ES2022 ----
    function atMethod(n) {
        var len = this.length >>> 0;
        var k = Math.trunc(n) || 0;
        if (k < 0) k += len;
        if (k < 0 || k >= len) return undefined;
        return this[k];
    }

    if (typeof Array.prototype.at !== 'function') {
        Object.defineProperty(Array.prototype, 'at', {
            value: atMethod, writable: true, configurable: true
        });
    }
    if (typeof String.prototype.at !== 'function') {
        Object.defineProperty(String.prototype, 'at', {
            value: atMethod, writable: true, configurable: true
        });
    }

    // ---- 9. Array.prototype.flat, flatMap, findLast, findLastIndex ----
    if (typeof Array.prototype.flat !== 'function') {
        Object.defineProperty(Array.prototype, 'flat', {
            value: function (depth) {
                var d = depth === undefined ? 1 : Math.trunc(depth);
                var result = [];
                (function flatten(arr, level) {
                    for (var i = 0; i < arr.length; i++) {
                        if (Array.isArray(arr[i]) && level > 0) flatten(arr[i], level - 1);
                        else if (i in arr) result.push(arr[i]);
                    }
                })(this, d);
                return result;
            },
            writable: true,
            configurable: true
        });
    }

    if (typeof Array.prototype.flatMap !== 'function') {
        Object.defineProperty(Array.prototype, 'flatMap', {
            value: function (cb, thisArg) {
                if (typeof cb !== 'function') throw new TypeError('flatMap requires a function');
                var result = [];
                for (var i = 0; i < this.length; i++) {
                    if (i in this) {
                        var val = cb.call(thisArg, this[i], i, this);
                        if (Array.isArray(val)) {
                            for (var j = 0; j < val.length; j++) result.push(val[j]);
                        } else result.push(val);
                    }
                }
                return result;
            },
            writable: true,
            configurable: true
        });
    }

    if (typeof Array.prototype.findLast !== 'function') {
        Object.defineProperty(Array.prototype, 'findLast', {
            value: function (predicate, thisArg) {
                if (typeof predicate !== 'function') throw new TypeError('predicate must be a function');
                for (var i = this.length - 1; i >= 0; i--) {
                    if (predicate.call(thisArg, this[i], i, this)) return this[i];
                }
                return undefined;
            },
            writable: true,
            configurable: true
        });
    }

    if (typeof Array.prototype.findLastIndex !== 'function') {
        Object.defineProperty(Array.prototype, 'findLastIndex', {
            value: function (predicate, thisArg) {
                if (typeof predicate !== 'function') throw new TypeError('predicate must be a function');
                for (var i = this.length - 1; i >= 0; i--) {
                    if (predicate.call(thisArg, this[i], i, this)) return i;
                }
                return -1;
            },
            writable: true,
            configurable: true
        });
    }

    // ---- 10. ES2023 Change-Array-by-Copy: toSorted, toReversed, toSpliced, with ----
    if (typeof Array.prototype.toSorted !== 'function') {
        Object.defineProperty(Array.prototype, 'toSorted', {
            value: function (compareFn) { return Array.from(this).sort(compareFn); },
            writable: true, configurable: true
        });
    }
    if (typeof Array.prototype.toReversed !== 'function') {
        Object.defineProperty(Array.prototype, 'toReversed', {
            value: function () { return Array.from(this).reverse(); },
            writable: true, configurable: true
        });
    }
    if (typeof Array.prototype.toSpliced !== 'function') {
        Object.defineProperty(Array.prototype, 'toSpliced', {
            value: function () {
                var copy = Array.from(this);
                Array.prototype.splice.apply(copy, arguments);
                return copy;
            },
            writable: true, configurable: true
        });
    }
    if (typeof Array.prototype.with !== 'function') {
        Object.defineProperty(Array.prototype, 'with', {
            value: function (index, value) {
                var copy = Array.from(this);
                var len = copy.length;
                var k = Math.trunc(index) || 0;
                if (k < 0) k += len;
                if (k < 0 || k >= len) throw new RangeError('Invalid index');
                copy[k] = value;
                return copy;
            },
            writable: true, configurable: true
        });
    }

    // ---- 11. String.prototype.replaceAll ----
    if (typeof String.prototype.replaceAll !== 'function') {
        Object.defineProperty(String.prototype, 'replaceAll', {
            value: function (search, replacement) {
                var str = String(this);
                if (search instanceof RegExp) {
                    if (!search.global) throw new TypeError('replaceAll requires a global RegExp');
                    return str.replace(search, replacement);
                }
                var escaped = String(search).replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
                return str.replace(new RegExp(escaped, 'g'), replacement);
            },
            writable: true, configurable: true
        });
    }

    // ---- 12. structuredClone() ----
    if (typeof globalThis.structuredClone !== 'function') {
        globalThis.structuredClone = function structuredClone(value) {
            var seen = new WeakMap();
            function clone(val) {
                if (val === null || typeof val !== 'object') return val;
                if (seen.has(val)) return seen.get(val);
                if (val instanceof Date) return new Date(val.getTime());
                if (val instanceof RegExp) return new RegExp(val.source, val.flags);
                if (val instanceof ArrayBuffer) return val.slice(0);
                if (ArrayBuffer.isView(val)) {
                    return new val.constructor(val.buffer.slice(val.byteOffset, val.byteOffset + val.byteLength));
                }
                if (val instanceof Map) {
                    var m = new Map();
                    seen.set(val, m);
                    val.forEach(function (v, k) { m.set(clone(k), clone(v)); });
                    return m;
                }
                if (val instanceof Set) {
                    var s = new Set();
                    seen.set(val, s);
                    val.forEach(function (v) { s.add(clone(v)); });
                    return s;
                }
                if (Array.isArray(val)) {
                    var arr = new Array(val.length);
                    seen.set(val, arr);
                    for (var i = 0; i < val.length; i++) {
                        if (i in val) arr[i] = clone(val[i]);
                    }
                    return arr;
                }
                var out = {};
                seen.set(val, out);
                var keys = Object.keys(val);
                for (var j = 0; j < keys.length; j++) {
                    var key = keys[j];
                    out[key] = clone(val[key]);
                }
                return out;
            }
            return clone(value);
        };
    }

    // ---- 13. crypto.randomUUID() & URL.canParse() ----
    if (typeof crypto !== 'undefined' &&
        typeof crypto.randomUUID !== 'function' &&
        typeof crypto.getRandomValues === 'function') {
        Object.defineProperty(crypto, 'randomUUID', {
            value: function () {
                var bytes = new Uint8Array(16);
                crypto.getRandomValues(bytes);
                bytes[6] = (bytes[6] & 0x0f) | 0x40;
                bytes[8] = (bytes[8] & 0x3f) | 0x80;
                var hex = [];
                for (var i = 0; i < 16; i++) hex.push((bytes[i] + 0x100).toString(16).slice(1));
                return hex.slice(0, 4).join('') + '-' + hex.slice(4, 6).join('') + '-' +
                       hex.slice(6, 8).join('') + '-' + hex.slice(8, 10).join('') + '-' +
                       hex.slice(10, 16).join('');
            },
            writable: true, configurable: true
        });
    }

    if (typeof URL !== 'undefined' && typeof URL.canParse !== 'function') {
        Object.defineProperty(URL, 'canParse', {
            value: function (url, base) {
                try {
                    if (base !== undefined) new URL(url, base);
                    else new URL(url);
                    return true;
                } catch (_) {
                    return false;
                }
            },
            writable: true, configurable: true
        });
    }

    // ---- 14. requestIdleCallback() ----
    if (typeof requestIdleCallback !== 'function') {
        globalThis.requestIdleCallback = function (cb) {
            var start = Date.now();
            return setTimeout(function () {
                cb({
                    didTimeout: false,
                    timeRemaining: function () {
                        return Math.max(0, 50 - (Date.now() - start));
                    }
                });
            }, 1);
        };
        globalThis.cancelIdleCallback = function (handle) { clearTimeout(handle); };
    }

    // ---- 15. Lightweight Batched ResizeObserver & IntersectionObserver (Không gọi getBoundingClientRect) ----
    var DEFAULT_RECT = { x: 0, y: 0, width: 100, height: 100, top: 0, left: 0, right: 100, bottom: 100 };
    var DEFAULT_BOX = [{ inlineSize: 100, blockSize: 100 }];

    if (typeof globalThis.ResizeObserver === 'undefined') {
        globalThis.ResizeObserver = function ResizeObserver(callback) {
            this._cb = callback;
            this._pending = [];
            this._scheduled = false;
        };
        globalThis.ResizeObserver.prototype.observe = function (target) {
            if (!target || typeof this._cb !== 'function') return;
            this._pending.push(target);
            if (!this._scheduled) {
                this._scheduled = true;
                var self = this;
                setTimeout(function () {
                    self._scheduled = false;
                    var list = self._pending.splice(0, self._pending.length);
                    if (list.length === 0) return;
                    var entries = list.map(function (el) {
                        return {
                            target: el,
                            contentRect: DEFAULT_RECT,
                            borderBoxSize: DEFAULT_BOX,
                            contentBoxSize: DEFAULT_BOX,
                            devicePixelContentBoxSize: DEFAULT_BOX
                        };
                    });
                    try { self._cb(entries, self); } catch (_) {}
                }, 16);
            }
        };
        globalThis.ResizeObserver.prototype.unobserve = function () {};
        globalThis.ResizeObserver.prototype.disconnect = function () {
            this._pending.length = 0;
        };
    }

    if (typeof globalThis.IntersectionObserver === 'undefined') {
        globalThis.IntersectionObserver = function IntersectionObserver(callback, options) {
            this._cb = callback;
            this._pending = [];
            this._scheduled = false;
            this.root = (options && options.root) || null;
            this.rootMargin = (options && options.rootMargin) || '0px';
            this.thresholds = [0];
        };
        globalThis.IntersectionObserver.prototype.observe = function (target) {
            if (!target || typeof this._cb !== 'function') return;
            this._pending.push(target);
            if (!this._scheduled) {
                this._scheduled = true;
                var self = this;
                setTimeout(function () {
                    self._scheduled = false;
                    var list = self._pending.splice(0, self._pending.length);
                    if (list.length === 0) return;
                    var now = Date.now();
                    var entries = list.map(function (el) {
                        return {
                            time: now,
                            target: el,
                            isIntersecting: true,
                            intersectionRatio: 1,
                            boundingClientRect: DEFAULT_RECT,
                            intersectionRect: DEFAULT_RECT,
                            rootBounds: null
                        };
                    });
                    try { self._cb(entries, self); } catch (_) {}
                }, 16);
            }
        };
        globalThis.IntersectionObserver.prototype.unobserve = function () {};
        globalThis.IntersectionObserver.prototype.disconnect = function () {
            this._pending.length = 0;
        };
        globalThis.IntersectionObserver.prototype.takeRecords = function () { return []; };
    }

    // ---- 16. PerformanceObserver & Element.animate Stub ----
    if (typeof globalThis.PerformanceObserver === 'undefined') {
        globalThis.PerformanceObserver = function PerformanceObserver() {};
        globalThis.PerformanceObserver.supportedEntryTypes = [];
        globalThis.PerformanceObserver.prototype.observe = function () {};
        globalThis.PerformanceObserver.prototype.disconnect = function () {};
        globalThis.PerformanceObserver.prototype.takeRecords = function () { return []; };
    }

    if (typeof Element !== 'undefined' && typeof Element.prototype.animate !== 'function') {
        Element.prototype.animate = function () {
            var anim = {
                playState: 'finished',
                finished: Promise.resolve(),
                onfinish: null,
                cancel: function () {},
                finish: function () {},
                pause: function () {},
                play: function () {},
                addEventListener: function () {},
                removeEventListener: function () {}
            };
            return anim;
        };
    }

    console.log('[Kestrel] polyfills & cosmetic adblock CSS loaded');
})();
"#;
