//! Polyfill JavaScript chuẩn hóa (v4) + Cosmetic Adblock CSS cho Servo Engine.
//! - Feature-detected 100%: Không bao giờ ghi đè các API mà Servo đã hỗ trợ native.
//! - Không gọi `getBoundingClientRect()` hàng loạt trong Observer để tránh nghẽn IPC Layout.
//! - Tương thích React 18/19 SSR Hydration (chỉ gắn style vào `document.head` hoặc `adoptedStyleSheets`).

pub const POLYFILL_SCRIPT: &str = r#"
(function () {
    'use strict';

    // ---- 1. globalThis ----
    if (typeof globalThis === 'undefined') {
        if (typeof window !== 'undefined') window.globalThis = window;
        else if (typeof self !== 'undefined') self.globalThis = self;
    }

    // ---- 2. Cosmetic Adblock CSS (Ẩn khung quảng cáo rỗng, an toàn với React Hydration) ----
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
        '.header-banner-ads,',
        /* Admicro / VCCorp (Dân Trí, Kenh14, CafeF, Tuổi Trẻ, Thanh Niên) */
        '[id^="adm-"], [id^="adm_"], [id^="admicro"], [class*="admicro"], .adm-ads, .admzone,',
        '[id^="admzone"], iframe[src*="admicro.vn"], iframe[src*="vcmedia.vn/ads"],',
        '.ads-zone, .qc-box, .box-qc, .banner-qc, .quang-cao, .box-quang-cao,',
        /* Taboola, Outbrain, MGID, Criteo & Generic EasyList Containers */
        '.taboola, .outbrain, [id^="taboola-"], [class*="outbrain"], [id^="mgid"], [class*="mgid"],',
        'iframe[src*="criteo."], .ad-banner, .ad-slot, .ad-placeholder,',
        '.ad-leaderboard, .ad-billboard, .sponsored-content',
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
        if (typeof document === 'undefined' || typeof location === 'undefined') return;
        var proto = location.protocol || '';
        if (proto === 'kestrel:' || proto === 'about:') return;
        if (document.getElementById('kestrel-cosmetic-adblock')) return;

        // Chỉ chèn khi document.head đã sẵn sàng để không làm lệch DOM hydration của React/Next.js
        if (!document.head) return;
        try {
            var style = document.createElement('style');
            style.id = 'kestrel-cosmetic-adblock';
            style.setAttribute('data-kestrel', 'cosmetic');
            style.textContent = COSMETIC_ADBLOCK_CSS;
            document.head.appendChild(style);
        } catch (_) {}
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

    // ---- 4. Promise.allSettled(), Promise.any() & AggregateError ----
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

    // ---- 6. Object.hasOwn(), Object.fromEntries(), Object.groupBy(), Map.groupBy() ----
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

    // ---- 7. Array, String & TypedArray Indexing / Copy Methods ----
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
            if (typeof TypedArrayProto.toReversed !== 'function') {
                Object.defineProperty(TypedArrayProto, 'toReversed', {
                    value: function () { return new this.constructor(this).reverse(); },
                    writable: true, configurable: true
                });
            }
            if (typeof TypedArrayProto.toSorted !== 'function') {
                Object.defineProperty(TypedArrayProto, 'toSorted', {
                    value: function (compareFn) { return new this.constructor(this).sort(compareFn); },
                    writable: true, configurable: true
                });
            }
            if (typeof TypedArrayProto.with !== 'function') {
                Object.defineProperty(TypedArrayProto, 'with', {
                    value: function (index, value) {
                        var copy = new this.constructor(this);
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

    // ---- 8. ES2024 Set Operations ----
    if (typeof Set !== 'undefined') {
        if (typeof Set.prototype.union !== 'function') {
            Object.defineProperty(Set.prototype, 'union', {
                value: function (other) {
                    var out = new Set(this);
                    if (other && typeof other.keys === 'function') {
                        var it = other.keys();
                        for (var step = it.next(); !step.done; step = it.next()) out.add(step.value);
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
        if (typeof Set.prototype.symmetricDifference !== 'function') {
            Object.defineProperty(Set.prototype, 'symmetricDifference', {
                value: function (other) {
                    var out = new Set(this);
                    if (other && typeof other.keys === 'function') {
                        var it = other.keys();
                        for (var step = it.next(); !step.done; step = it.next()) {
                            var v = step.value;
                            if (this.has(v)) out.delete(v);
                            else out.add(v);
                        }
                    }
                    return out;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.isSubsetOf !== 'function') {
            Object.defineProperty(Set.prototype, 'isSubsetOf', {
                value: function (other) {
                    if (!other || typeof other.has !== 'function') return false;
                    var ok = true;
                    this.forEach(function (v) { if (!other.has(v)) ok = false; });
                    return ok;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.isSupersetOf !== 'function') {
            Object.defineProperty(Set.prototype, 'isSupersetOf', {
                value: function (other) {
                    if (!other || typeof other.keys !== 'function') return false;
                    var it = other.keys();
                    for (var step = it.next(); !step.done; step = it.next()) {
                        if (!this.has(step.value)) return false;
                    }
                    return true;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.isDisjointFrom !== 'function') {
            Object.defineProperty(Set.prototype, 'isDisjointFrom', {
                value: function (other) {
                    if (!other || typeof other.has !== 'function') return true;
                    var disjoint = true;
                    this.forEach(function (v) { if (other.has(v)) disjoint = false; });
                    return disjoint;
                },
                writable: true, configurable: true
            });
        }
    }

    // ---- 9. String.prototype.replaceAll, isWellFormed, toWellFormed ----
    if (typeof String.prototype.replaceAll !== 'function') {
        Object.defineProperty(String.prototype, 'replaceAll', {
            value: function (search, replacement) {
                var str = String(this);
                if (search instanceof RegExp) {
