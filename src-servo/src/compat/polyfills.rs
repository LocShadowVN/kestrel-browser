//! Polyfill JavaScript mở rộng cho Servo Engine.
//!
//! Nguyên tắc:
//! - Luôn kiểm tra feature-detection (`typeof ... === 'undefined'`) trước khi định nghĩa,
//!   tuyệt đối không ghi đè implementation native khi Servo đã hỗ trợ.
//! - Bổ sung đầy đủ các API ES2021–ES2024 và Web/DOM API phổ biến mà các framework hiện đại
//!   (React, Vue, Next.js, Svelte, Radix UI, Tailwind) yêu cầu để tránh lỗi màn hình trắng.

pub const POLYFILL_SCRIPT: &str = r#"
(function () {
    'use strict';

    // ---- 1. globalThis ----
    if (typeof globalThis === 'undefined') {
        if (typeof window !== 'undefined') window.globalThis = window;
        else if (typeof self !== 'undefined') self.globalThis = self;
    }

    // ---- 2. Promise.withResolvers() — ES2024 ----
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

    // ---- 3. Promise.allSettled() & Promise.any() — ES2020/ES2021 ----
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

    // ---- 4. queueMicrotask() ----
    if (typeof queueMicrotask !== 'function') {
        globalThis.queueMicrotask = function (cb) {
            if (typeof cb !== 'function') throw new TypeError('queueMicrotask requires a function');
            Promise.resolve().then(cb).catch(function (e) {
                setTimeout(function () { throw e; }, 0);
            });
        };
    }

    // ---- 5. Object.hasOwn() (ES2022) & Object.fromEntries() (ES2019) ----
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

    // ---- 6. Object.groupBy() & Map.groupBy() — ES2024 ----
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

    // ---- 7. Array.prototype.at, String.prototype.at, TypedArray.prototype.at — ES2022 ----
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
    if (typeof Uint8Array !== 'undefined') {
        var typedArrayProto = Object.getPrototypeOf(Uint8Array.prototype);
        if (typedArrayProto && typeof typedArrayProto.at !== 'function') {
            Object.defineProperty(typedArrayProto, 'at', {
                value: atMethod, writable: true, configurable: true
            });
        }
    }

    // ---- 8. Array.prototype.flat & flatMap (ES2019) ----
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

    // ---- 9. Array.prototype.findLast & findLastIndex (ES2023) ----
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
            value: function (compareFn) {
                return Array.from(this).sort(compareFn);
            },
            writable: true,
            configurable: true
        });
    }

    if (typeof Array.prototype.toReversed !== 'function') {
        Object.defineProperty(Array.prototype, 'toReversed', {
            value: function () {
                return Array.from(this).reverse();
            },
            writable: true,
            configurable: true
        });
    }

    if (typeof Array.prototype.toSpliced !== 'function') {
        Object.defineProperty(Array.prototype, 'toSpliced', {
            value: function () {
                var copy = Array.from(this);
                Array.prototype.splice.apply(copy, arguments);
                return copy;
            },
            writable: true,
            configurable: true
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
            writable: true,
            configurable: true
        });
    }

    // ---- 11. ES2024 Set Methods ----
    if (typeof Set !== 'undefined') {
        if (typeof Set.prototype.union !== 'function') {
            Object.defineProperty(Set.prototype, 'union', {
                value: function (other) {
                    var res = new Set(this);
                    for (var v of other) res.add(v);
                    return res;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.intersection !== 'function') {
            Object.defineProperty(Set.prototype, 'intersection', {
                value: function (other) {
                    var res = new Set();
                    for (var v of other) {
                        if (this.has(v)) res.add(v);
                    }
                    return res;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.difference !== 'function') {
            Object.defineProperty(Set.prototype, 'difference', {
                value: function (other) {
                    var res = new Set(this);
                    for (var v of other) res.delete(v);
                    return res;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.symmetricDifference !== 'function') {
            Object.defineProperty(Set.prototype, 'symmetricDifference', {
                value: function (other) {
                    var res = new Set(this);
                    for (var v of other) {
                        if (res.has(v)) res.delete(v);
                        else res.add(v);
                    }
                    return res;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.isSubsetOf !== 'function') {
            Object.defineProperty(Set.prototype, 'isSubsetOf', {
                value: function (other) {
                    var otherSet = other instanceof Set ? other : new Set(other);
                    for (var v of this) {
                        if (!otherSet.has(v)) return false;
                    }
                    return true;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.isSupersetOf !== 'function') {
            Object.defineProperty(Set.prototype, 'isSupersetOf', {
                value: function (other) {
                    for (var v of other) {
                        if (!this.has(v)) return false;
                    }
                    return true;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.isDisjointFrom !== 'function') {
            Object.defineProperty(Set.prototype, 'isDisjointFrom', {
                value: function (other) {
                    for (var v of other) {
                        if (this.has(v)) return false;
                    }
                    return true;
                },
                writable: true, configurable: true
            });
        }
    }

    // ---- 12. String.prototype.replaceAll (ES2021) ----
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
            writable: true,
            configurable: true
        });
    }

    // ---- 13. structuredClone() với hỗ trợ Date, RegExp, Map, Set, ArrayBuffer, TypedArray & Cycles ----
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

    // ---- 14. crypto.randomUUID() ----
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
            writable: true,
            configurable: true
        });
    }

    // ---- 15. URL.canParse() — WHATWG URL ----
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
            writable: true,
            configurable: true
        });
    }

    // ---- 16. AbortSignal.timeout() & AbortSignal.any() ----
    if (typeof AbortSignal !== 'undefined' && typeof AbortController !== 'undefined') {
        if (typeof AbortSignal.timeout !== 'function') {
            Object.defineProperty(AbortSignal, 'timeout', {
                value: function (ms) {
                    var ctrl = new AbortController();
                    setTimeout(function () {
                        var err = typeof DOMException !== 'undefined'
                            ? new DOMException('Signal timed out', 'TimeoutError')
                            : new Error('TimeoutError');
                        ctrl.abort(err);
                    }, ms);
                    return ctrl.signal;
                },
                writable: true,
                configurable: true
            });
        }
        if (typeof AbortSignal.any !== 'function') {
            Object.defineProperty(AbortSignal, 'any', {
                value: function (signals) {
                    var ctrl = new AbortController();
                    for (var sig of signals) {
                        if (sig && sig.aborted) {
                            ctrl.abort(sig.reason);
                            return ctrl.signal;
                        }
                        if (sig && typeof sig.addEventListener === 'function') {
                            sig.addEventListener('abort', function () {
                                ctrl.abort(sig.reason);
                            }, { once: true });
                        }
                    }
                    return ctrl.signal;
                },
                writable: true,
                configurable: true
            });
        }
    }

    // ---- 17. requestIdleCallback() / cancelIdleCallback() ----
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

    // ---- 18. ResizeObserver Fallback (Ngăn crash SPA React/Vue/Radix UI) ----
    if (typeof globalThis.ResizeObserver === 'undefined') {
        globalThis.ResizeObserver = function ResizeObserver(callback) {
            this._cb = callback;
            this._targets = new Set();
            var self = this;
            this._onResize = function () { self._notify(); };
        };
        globalThis.ResizeObserver.prototype.observe = function (target) {
            if (!target) return;
            if (this._targets.size === 0 && typeof window !== 'undefined') {
                window.addEventListener('resize', this._onResize);
            }
            this._targets.add(target);
            var self = this;
            setTimeout(function () { self._notifyTarget(target); }, 0);
        };
        globalThis.ResizeObserver.prototype.unobserve = function (target) {
            this._targets.delete(target);
            if (this._targets.size === 0 && typeof window !== 'undefined') {
                window.removeEventListener('resize', this._onResize);
            }
        };
        globalThis.ResizeObserver.prototype.disconnect = function () {
            this._targets.clear();
            if (typeof window !== 'undefined') {
                window.removeEventListener('resize', this._onResize);
            }
        };
        globalThis.ResizeObserver.prototype._makeEntry = function (el) {
            var rect = (el && typeof el.getBoundingClientRect === 'function')
                ? el.getBoundingClientRect()
                : { x: 0, y: 0, width: 0, height: 0, top: 0, left: 0, right: 0, bottom: 0 };
            var box = [{ inlineSize: rect.width || 0, blockSize: rect.height || 0 }];
            return {
                target: el,
                contentRect: rect,
                borderBoxSize: box,
                contentBoxSize: box,
                devicePixelContentBoxSize: box
            };
        };
        globalThis.ResizeObserver.prototype._notifyTarget = function (el) {
            if (!this._targets.has(el) || typeof this._cb !== 'function') return;
            try { this._cb([this._makeEntry(el)], this); } catch (_) {}
        };
        globalThis.ResizeObserver.prototype._notify = function () {
            if (this._targets.size === 0 || typeof this._cb !== 'function') return;
            var entries = [];
            var self = this;
            this._targets.forEach(function (el) { entries.push(self._makeEntry(el)); });
            try { this._cb(entries, this); } catch (_) {}
        };
    }

    // ---- 19. IntersectionObserver Fallback (Giúp ảnh lazy-load & infinite scroll hiển thị) ----
    if (typeof globalThis.IntersectionObserver === 'undefined') {
        globalThis.IntersectionObserver = function IntersectionObserver(callback, options) {
            this._cb = callback;
            this._targets = new Set();
            this.root = (options && options.root) || null;
            this.rootMargin = (options && options.rootMargin) || '0px';
            this.thresholds = [0];
        };
        globalThis.IntersectionObserver.prototype.observe = function (target) {
            if (!target) return;
            this._targets.add(target);
            var self = this;
            setTimeout(function () {
                if (!self._targets.has(target) || typeof self._cb !== 'function') return;
                var rect = typeof target.getBoundingClientRect === 'function'
                    ? target.getBoundingClientRect()
                    : { x: 0, y: 0, width: 100, height: 100, top: 0, left: 0, right: 100, bottom: 100 };
                var entry = {
                    time: typeof performance !== 'undefined' ? performance.now() : Date.now(),
                    target: target,
                    isIntersecting: true,
                    intersectionRatio: 1,
                    boundingClientRect: rect,
                    intersectionRect: rect,
                    rootBounds: null
                };
                try { self._cb([entry], self); } catch (_) {}
            }, 0);
        };
        globalThis.IntersectionObserver.prototype.unobserve = function (target) {
            this._targets.delete(target);
        };
        globalThis.IntersectionObserver.prototype.disconnect = function () {
            this._targets.clear();
        };
        globalThis.IntersectionObserver.prototype.takeRecords = function () {
            return [];
        };
    }

    // ---- 20. PerformanceObserver Fallback (Ngăn crash từ Next.js / Core Web Vitals / Sentry) ----
    if (typeof globalThis.PerformanceObserver === 'undefined') {
        globalThis.PerformanceObserver = function PerformanceObserver(callback) {
            this._cb = callback;
        };
        globalThis.PerformanceObserver.supportedEntryTypes = [];
        globalThis.PerformanceObserver.prototype.observe = function () {};
        globalThis.PerformanceObserver.prototype.disconnect = function () {};
        globalThis.PerformanceObserver.prototype.takeRecords = function () { return []; };
    }

    // ---- 21. window.visualViewport Fallback (Ngăn crash trên Floating UI / Popper / Radix) ----
    if (typeof window !== 'undefined' && typeof window.visualViewport === 'undefined') {
        var vvListeners = {};
        window.visualViewport = {
            get width() { return window.innerWidth || 1280; },
            get height() { return window.innerHeight || 800; },
            offsetLeft: 0,
            offsetTop: 0,
            pageLeft: 0,
            pageTop: 0,
            scale: 1,
            addEventListener: function (type, cb) {
                if (!vvListeners[type]) vvListeners[type] = [];
                vvListeners[type].push(cb);
                if (type === 'resize' || type === 'scroll') {
                    window.addEventListener(type, cb);
                }
            },
            removeEventListener: function (type, cb) {
                if (type === 'resize' || type === 'scroll') {
                    window.removeEventListener(type, cb);
                }
            }
        };
    }

    // ---- 22. BroadcastChannel Fallback (Ngăn crash trên NextAuth / Supabase / Firebase) ----
    if (typeof globalThis.BroadcastChannel === 'undefined') {
        var channels = {};
        globalThis.BroadcastChannel = function BroadcastChannel(name) {
            this.name = String(name);
            this.onmessage = null;
            this.onmessageerror = null;
            this._listeners = [];
            if (!channels[this.name]) channels[this.name] = [];
            channels[this.name].push(this);
        };
        globalThis.BroadcastChannel.prototype.postMessage = function (msg) {
            var self = this;
            var list = channels[this.name] || [];
            setTimeout(function () {
                list.forEach(function (peer) {
                    if (peer === self) return;
                    var evt = { data: msg, target: peer };
                    if (typeof peer.onmessage === 'function') peer.onmessage(evt);
                    peer._listeners.forEach(function (fn) { fn(evt); });
                });
            }, 0);
        };
        globalThis.BroadcastChannel.prototype.addEventListener = function (type, fn) {
            if (type === 'message' && typeof fn === 'function') this._listeners.push(fn);
        };
        globalThis.BroadcastChannel.prototype.removeEventListener = function (type, fn) {
            if (type === 'message') {
                this._listeners = this._listeners.filter(function (f) { return f !== fn; });
            }
        };
        globalThis.BroadcastChannel.prototype.close = function () {
            var self = this;
            if (channels[this.name]) {
                channels[this.name] = channels[this.name].filter(function (p) { return p !== self; });
            }
        };
    }

    // ---- 23. window.matchMedia Dual Listener Compatibility ----
    if (typeof window !== 'undefined') {
        if (typeof window.matchMedia !== 'function') {
            window.matchMedia = function (query) {
                return {
                    matches: false,
                    media: String(query),
                    onchange: null,
                    addListener: function () {},
                    removeListener: function () {},
                    addEventListener: function () {},
                    removeEventListener: function () {},
                    dispatchEvent: function () { return false; }
                };
            };
        } else {
            var origMatchMedia = window.matchMedia.bind(window);
            window.matchMedia = function (query) {
                var mql = origMatchMedia(query);
                if (mql) {
                    if (typeof mql.addEventListener !== 'function' && typeof mql.addListener === 'function') {
                        mql.addEventListener = function (type, listener) {
                            if (type === 'change') mql.addListener(listener);
                        };
                        mql.removeEventListener = function (type, listener) {
                            if (type === 'change') mql.removeListener(listener);
                        };
                    }
                    if (typeof mql.addListener !== 'function' && typeof mql.addEventListener === 'function') {
                        mql.addListener = function (listener) { mql.addEventListener('change', listener); };
                        mql.removeListener = function (listener) { mql.removeEventListener('change', listener); };
                    }
                }
                return mql;
            };
        }
    }

    // ---- 24. Element.prototype.animate / getAnimations / checkVisibility ----
    if (typeof Element !== 'undefined') {
        if (typeof Element.prototype.animate !== 'function') {
            Element.prototype.animate = function () {
                var anim = {
                    playState: 'finished',
                    finished: Promise.resolve(),
                    onfinish: null,
                    oncancel: null,
                    cancel: function () {},
                    finish: function () {},
                    pause: function () {},
                    play: function () {},
                    reverse: function () {},
                    addEventListener: function (type, fn) {
                        if (type === 'finish' && typeof fn === 'function') setTimeout(fn, 0);
                    },
                    removeEventListener: function () {}
                };
                setTimeout(function () {
                    if (typeof anim.onfinish === 'function') anim.onfinish();
                }, 0);
                return anim;
            };
        }
        if (typeof Element.prototype.getAnimations !== 'function') {
            Element.prototype.getAnimations = function () { return []; };
        }
        if (typeof Element.prototype.checkVisibility !== 'function') {
            Element.prototype.checkVisibility = function () {
                if (!this.isConnected) return false;
                var style = typeof getComputedStyle === 'function' ? getComputedStyle(this) : null;
                if (style && (style.display === 'none' || style.visibility === 'hidden')) return false;
                return true;
            };
        }
        if (typeof Element.prototype.scrollIntoViewIfNeeded !== 'function') {
            Element.prototype.scrollIntoViewIfNeeded = function () {
                if (typeof this.scrollIntoView === 'function') this.scrollIntoView();
            };
        }
    }

    // ---- 25. CSS.supports() & CSS.escape() ----
    if (typeof globalThis.CSS === 'undefined') {
        globalThis.CSS = {};
    }
    if (typeof globalThis.CSS.supports !== 'function') {
        globalThis.CSS.supports = function () { return true; };
    }
    if (typeof globalThis.CSS.escape !== 'function') {
        globalThis.CSS.escape = function (value) {
            return String(value).replace(/([^\w-])/g, '\\$1');
        };
    }

    // ---- 26. navigator.clipboard & sendBeacon ----
    if (typeof navigator !== 'undefined') {
        if (!navigator.clipboard) {
            var clipText = '';
            try {
                Object.defineProperty(navigator, 'clipboard', {
                    value: {
                        writeText: function (text) {
                            clipText = String(text);
                            return Promise.resolve();
                        },
                        readText: function () {
                            return Promise.resolve(clipText);
                        }
                    },
                    configurable: true
                });
            } catch (_) {}
        }
        if (typeof navigator.sendBeacon !== 'function') {
            try {
                navigator.sendBeacon = function () { return true; };
            } catch (_) {}
        }
    }

    console.log('[Kestrel] Web compatibility polyfills v3 loaded');
})();
"#;
