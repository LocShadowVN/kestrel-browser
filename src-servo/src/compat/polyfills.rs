//! Polyfill JavaScript chuẩn hóa (v5) + Active Cosmetic & DOM Adblock cho Servo Engine.
//! - Tự động gắn CSS ẩn quảng cáo kể cả khi `document.head` chưa kịp tạo lúc khởi chạy script.
//! - Quét và thu gọn triệt để các khung quảng cáo rỗng (VnExpress/Eclick, Dân Trí/Admicro, Google Ads...)
//!   đồng thời gửi số lượng phần tử đã chặn về bộ đếm Kestrel Shields.

pub const POLYFILL_SCRIPT: &str = r#"
(function () {
    'use strict';

    // ---- 1. globalThis ----
    if (typeof globalThis === 'undefined') {
        if (typeof window !== 'undefined') window.globalThis = window;
        else if (typeof self !== 'undefined') self.globalThis = self;
    }

    // ---- 2. Active Cosmetic Adblock & DOM Ad Scrubber ----
    var AD_SELECTORS = [
        'ins.adsbygoogle',
        '.adsbygoogle',
        '[id^="div-gpt-ad"]',
        '[id^="google_ads_iframe"]',
        'iframe[id^="google_ads_"]',
        'iframe[src*="doubleclick.net"]',
        'iframe[src*="googlesyndication.com"]',
        'iframe[src*="adservice.google."]',
        '[data-ad-client]',
        '[data-ad-slot]',
        '[data-google-query-id]',
        /* VnExpress & FPT Eclick */
        '#sis_banner_top',
        '[id^="sis_"]',
        '[class^="sis_"]',
        '[id^="eclick"]',
        '[class*="eclick"]',
        'iframe[src*="eclick.vn"]',
        'script[src*="eclick.vn"]',
        '.banner-ads',
        '.box-ads',
        '.ads-top',
        '.ads-right',
        '.ads-middle',
        '.ads-bottom',
        '.ads-sticky',
        '.section_banner_top',
        '.section-banner-top',
        '.banner_top',
        '.wrap-banner-top',
        '.lazy-ads',
        '[id^="banner_top"]',
        '[id^="ads_"]',
        '[id*="ads_banner"]',
        '[class*="banner_ads"]',
        '.item_box_ads',
        '.box_quangcao',
        '.block_ads',
        '.header-banner-ads',
        '[data-component*="BannerAds"]',
        /* Admicro / VCCorp (Dân Trí, Kenh14, CafeF, Tuổi Trẻ, Thanh Niên) */
        '[id^="adm-"]',
        '[id^="adm_"]',
        '[id^="admicro"]',
        '[class*="admicro"]',
        '.adm-ads',
        '.admzone',
        '[id^="admzone"]',
        'iframe[src*="admicro.vn"]',
        'script[src*="admicro.vn"]',
        'iframe[src*="vcmedia.vn/ads"]',
        '.ads-zone',
        '.qc-box',
        '.box-qc',
        '.banner-qc',
        '.quang-cao',
        '.box-quang-cao',
        /* Quốc tế: Taboola, Outbrain, MGID, Criteo */
        '.taboola',
        '.outbrain',
        '[id^="taboola-"]',
        '[class*="outbrain"]',
        '[id^="mgid"]',
        '[class*="mgid"]',
        'iframe[src*="criteo."]',
        '.ad-banner',
        '.ad-slot',
        '.ad-placeholder',
        '.ad-leaderboard',
        '.ad-billboard',
        '.sponsored-content'
    ];

    var COSMETIC_ADBLOCK_CSS = AD_SELECTORS.join(',\n') + ' {\n' +
        '  display: none !important;\n' +
        '  visibility: hidden !important;\n' +
        '  width: 0 !important;\n' +
        '  height: 0 !important;\n' +
        '  min-height: 0 !important;\n' +
        '  max-height: 0 !important;\n' +
        '  margin: 0 !important;\n' +
        '  padding: 0 !important;\n' +
        '  border: none !important;\n' +
        '  overflow: hidden !important;\n' +
        '  pointer-events: none !important;\n' +
        '}\n';

    function isInternalPage() {
        if (typeof location === 'undefined') return true;
        var p = location.protocol || '';
        var h = location.href || '';
        return p === 'kestrel:' || p === 'about:' || p === 'data:' || h.indexOf('kestrel://') === 0;
    }

    function reportCosmeticBlocked(count) {
        if (count <= 0) return;
        try {
            if (window.servoGtk && window.servoGtk.messageHandlers &&
                window.servoGtk.messageHandlers.kestrel) {
                window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify({
                    action: 'adblock-cosmetic-hit',
                    count: count
                }));
            }
        } catch (_) {}
    }

    function collapseElement(el) {
        if (!el || !el.style) return;
        el.style.setProperty('display', 'none', 'important');
        el.style.setProperty('height', '0px', 'important');
        el.style.setProperty('min-height', '0px', 'important');
        el.style.setProperty('margin', '0px', 'important');
        el.style.setProperty('padding', '0px', 'important');
    }

    var combinedSelector = AD_SELECTORS.join(',');

    function scrubAds() {
        if (typeof document === 'undefined' || isInternalPage()) return;

        if (!document.getElementById('kestrel-cosmetic-adblock')) {
            var target = document.head || document.documentElement;
            if (target) {
                try {
                    var style = document.createElement('style');
                    style.id = 'kestrel-cosmetic-adblock';
                    style.textContent = COSMETIC_ADBLOCK_CSS;
                    target.appendChild(style);
                } catch (_) {}
            }
        }

        if (!document.body) return;
        var newlyBlocked = 0;
        try {
            var nodes = document.querySelectorAll(combinedSelector);
            for (var i = 0; i < nodes.length; i++) {
                var el = nodes[i];
                if (el.getAttribute('data-kestrel-hidden') === '1') continue;
                el.setAttribute('data-kestrel-hidden', '1');
                newlyBlocked++;

                var tag = (el.tagName || '').toUpperCase();
                if (tag === 'IFRAME' || tag === 'SCRIPT' || tag === 'INS') {
                    var parent = el.parentElement;
                    try { el.remove(); } catch (_) { collapseElement(el); }
                    if (parent && parent !== document.body && parent.children.length === 0) {
                        collapseElement(parent);
                    }
                } else {
                    collapseElement(el);
                    var p = el.parentElement;
                    if (p && p !== document.body && p.children.length === 1) {
                        var cls = (p.className || '') + ' ' + (p.id || '');
                        if (/banner|ads|eclick|sis_|quangcao|qc/i.test(cls)) {
                            collapseElement(p);
                        }
                    }
                }
            }
        } catch (_) {}

        if (newlyBlocked > 0) {
            reportCosmeticBlocked(newlyBlocked);
        }
    }

    if (!isInternalPage()) {
        scrubAds();
        setTimeout(scrubAds, 60);
        setTimeout(scrubAds, 300);
        setTimeout(scrubAds, 1000);
        setTimeout(scrubAds, 2500);
        if (typeof document !== 'undefined') {
            document.addEventListener('DOMContentLoaded', scrubAds, { once: true });
        }
        if (typeof window !== 'undefined') {
            window.addEventListener('load', function () {
                scrubAds();
                setTimeout(scrubAds, 800);
            }, { once: true });
        }
    }

    // ---- 3. Promise.withResolvers(), allSettled(), any() & AggregateError ----
    if (typeof Promise !== 'undefined' && typeof Promise.withResolvers !== 'function') {
        Object.defineProperty(Promise, 'withResolvers', {
            value: function () {
                var resolve, reject;
                var promise = new Promise(function (res, rej) { resolve = res; reject = rej; });
                return { promise: promise, resolve: resolve, reject: reject };
            },
            writable: true, configurable: true
        });
    }

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
            writable: true, configurable: true
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
            writable: true, configurable: true
        });
    }

    // ---- 4. queueMicrotask() ----
    if (typeof queueMicrotask !== 'function') {
        globalThis.queueMicrotask = function (cb) {
            if (typeof cb !== 'function') throw new TypeError('queueMicrotask requires a function');
            Promise.resolve().then(cb).catch(function (e) {
                setTimeout(function () { throw e; }, 0);
            });
        };
    }

    // ---- 5. Object.hasOwn(), fromEntries(), groupBy() & Map.groupBy() ----
    if (typeof Object.hasOwn !== 'function') {
        Object.defineProperty(Object, 'hasOwn', {
            value: function (obj, prop) {
                if (obj === null || obj === undefined) {
                    throw new TypeError('Cannot convert undefined or null to object');
                }
                return Object.prototype.hasOwnProperty.call(Object(obj), prop);
            },
            writable: true, configurable: true
        });
    }

    if (typeof Object.fromEntries !== 'function') {
        Object.defineProperty(Object, 'fromEntries', {
            value: function (iterable) {
                var obj = {};
                for (var item of iterable) {
                    if (item && typeof item === 'object') obj[item[0]] = item[1];
                }
                return obj;
            },
            writable: true, configurable: true
        });
    }

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
            writable: true, configurable: true
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
                    if (!list) { list = []; map.set(key, list); }
                    list.push(item);
                }
                return map;
            },
            writable: true, configurable: true
        });
    }

    // ---- 6. Array, String & TypedArray Indexing / Copy Methods ----
    function atMethod(n) {
        var len = this.length >>> 0;
        var k = Math.trunc(n) || 0;
        if (k < 0) k += len;
        if (k < 0 || k >= len) return undefined;
        return this[k];
    }

    function findLastMethod(predicate, thisArg) {
        if (typeof predicate !== 'function') throw new TypeError('predicate must be a function');
        for (var i = (this.length >>> 0) - 1; i >= 0; i--) {
            if (predicate.call(thisArg, this[i], i, this)) return this[i];
        }
        return undefined;
    }

    function findLastIndexMethod(predicate, thisArg) {
        if (typeof predicate !== 'function') throw new TypeError('predicate must be a function');
        for (var i = (this.length >>> 0) - 1; i >= 0; i--) {
            if (predicate.call(thisArg, this[i], i, this)) return i;
        }
        return -1;
    }

    if (typeof Array.prototype.at !== 'function') {
        Object.defineProperty(Array.prototype, 'at', { value: atMethod, writable: true, configurable: true });
    }
    if (typeof String.prototype.at !== 'function') {
        Object.defineProperty(String.prototype, 'at', { value: atMethod, writable: true, configurable: true });
    }
    if (typeof Array.prototype.findLast !== 'function') {
        Object.defineProperty(Array.prototype, 'findLast', { value: findLastMethod, writable: true, configurable: true });
    }
    if (typeof Array.prototype.findLastIndex !== 'function') {
        Object.defineProperty(Array.prototype, 'findLastIndex', { value: findLastIndexMethod, writable: true, configurable: true });
    }

    if (typeof Uint8Array !== 'undefined') {
        var TypedArrayProto = Object.getPrototypeOf(Uint8Array.prototype);
        if (TypedArrayProto) {
            if (typeof TypedArrayProto.at !== 'function') {
                Object.defineProperty(TypedArrayProto, 'at', { value: atMethod, writable: true, configurable: true });
            }
            if (typeof TypedArrayProto.findLast !== 'function') {
                Object.defineProperty(TypedArrayProto, 'findLast', { value: findLastMethod, writable: true, configurable: true });
            }
            if (typeof TypedArrayProto.findLastIndex !== 'function') {
                Object.defineProperty(TypedArrayProto, 'findLastIndex', { value: findLastIndexMethod, writable: true, configurable: true });
            }
        }
    }

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
            writable: true, configurable: true
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
            writable: true, configurable: true
        });
    }

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

    // ---- 7. ES2024 Set Operations ----
    if (typeof Set !== 'undefined') {
        if (typeof Set.prototype.union !== 'function') {
            Object.defineProperty(Set.prototype, 'union', {
                value: function (other) {
                    var out = new Set(this);
                    if (other && typeof other.keys === 'function') {
                        var it = other.keys();
                        for (var s = it.next(); !s.done; s = it.next()) out.add(s.value);
                    }
                    return out;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.intersection !== 'function') {
            Object.defineProperty(Set.prototype, 'intersection', {
                value: function (other) {
                    var out = new Set();
                    this.forEach(function (v) {
                        if (other && typeof other.has === 'function' && other.has(v)) out.add(v);
                    });
                    return out;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.difference !== 'function') {
            Object.defineProperty(Set.prototype, 'difference', {
                value: function (other) {
                    var out = new Set(this);
                    this.forEach(function (v) {
                        if (other && typeof other.has === 'function' && other.has(v)) out.delete(v);
                    });
                    return out;
                },
                writable: true, configurable: true
            });
        }
    }

    // ---- 8. String.prototype.replaceAll ----
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

    // ---- 9. structuredClone() ----
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
                    if (val instanceof DataView) {
                        return new DataView(val.buffer.slice(val.byteOffset, val.byteOffset + val.byteLength));
                    }
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

    // ---- 10. crypto.randomUUID(), URL.canParse(), requestIdleCallback ----
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

    if (typeof requestIdleCallback !== 'function') {
        globalThis.requestIdleCallback = function (cb) {
            var start = Date.now();
            return setTimeout(function () {
                cb({
                    didTimeout: false,
                    timeRemaining: function () { return Math.max(0, 50 - (Date.now() - start)); }
                });
            }, 1);
        };
        globalThis.cancelIdleCallback = function (handle) { clearTimeout(handle); };
    }

    // ---- 11. window.matchMedia & window.visualViewport ----
    if (typeof window !== 'undefined') {
        if (typeof window.matchMedia !== 'function') {
            window.matchMedia = function (query) {
                return {
                    matches: false,
                    media: String(query || ''),
                    onchange: null,
                    addListener: function () {},
                    removeListener: function () {},
                    addEventListener: function () {},
                    removeEventListener: function () {},
                    dispatchEvent: function () { return true; }
                };
            };
        }
        if (typeof window.visualViewport === 'undefined') {
            var vv = {
                offsetLeft: 0, offsetTop: 0, pageLeft: 0, pageTop: 0, scale: 1,
                onresize: null, onscroll: null,
                addEventListener: function (type, fn, opts) { window.addEventListener(type, fn, opts); },
                removeEventListener: function (type, fn, opts) { window.removeEventListener(type, fn, opts); },
                dispatchEvent: function (ev) { return window.dispatchEvent(ev); }
            };
            Object.defineProperties(vv, {
                width: { get: function () { return window.innerWidth || 1280; }, configurable: true },
                height: { get: function () { return window.innerHeight || 800; }, configurable: true }
            });
            window.visualViewport = vv;
        }
    }

    // ---- 12. CSS.supports() ----
    if (typeof globalThis.CSS === 'undefined') globalThis.CSS = {};
    if (typeof globalThis.CSS.supports !== 'function') {
        globalThis.CSS.supports = function () { return true; };
    }

    // ---- 13. Lightweight Deduplicated ResizeObserver, IntersectionObserver & PerformanceObserver ----
    var DEFAULT_RECT = {
        x: 0, y: 0, width: 100, height: 100, top: 0, left: 0, right: 100, bottom: 100,
        toJSON: function () { return this; }
    };
    var DEFAULT_BOX = [{ inlineSize: 100, blockSize: 100 }];

    if (typeof globalThis.ResizeObserver === 'undefined') {
        globalThis.ResizeObserver = function ResizeObserver(callback) {
            this._cb = callback;
            this._pending = [];
            this._scheduled = false;
        };
        globalThis.ResizeObserver.prototype.observe = function (target) {
            if (!target || typeof this._cb !== 'function') return;
            if (this._pending.indexOf(target) === -1) this._pending.push(target);
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
        globalThis.ResizeObserver.prototype.unobserve = function (target) {
            var idx = this._pending.indexOf(target);
            if (idx !== -1) this._pending.splice(idx, 1);
        };
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
            if (this._pending.indexOf(target) === -1) this._pending.push(target);
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
        globalThis.IntersectionObserver.prototype.unobserve = function (target) {
            var idx = this._pending.indexOf(target);
            if (idx !== -1) this._pending.splice(idx, 1);
        };
        globalThis.IntersectionObserver.prototype.disconnect = function () {
            this._pending.length = 0;
        };
        globalThis.IntersectionObserver.prototype.takeRecords = function () { return []; };
    }

    if (typeof globalThis.PerformanceObserver === 'undefined') {
        globalThis.PerformanceObserver = function PerformanceObserver() {};
        globalThis.PerformanceObserver.supportedEntryTypes = [];
        globalThis.PerformanceObserver.prototype.observe = function () {};
        globalThis.PerformanceObserver.prototype.disconnect = function () {};
        globalThis.PerformanceObserver.prototype.takeRecords = function () { return []; };
    }

    // ---- 14. DOM & Web Animations Shims ----
    if (typeof Element !== 'undefined') {
        if (typeof Element.prototype.animate !== 'function') {
            Element.prototype.animate = function () {
                return {
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
            };
        }
        if (typeof Element.prototype.getAnimations !== 'function') {
            Element.prototype.getAnimations = function () { return []; };
        }
        if (typeof Element.prototype.checkVisibility !== 'function') {
            Element.prototype.checkVisibility = function () { return true; };
        }
    }
})();
"#;
