//! Polyfill JavaScript chuẩn hóa (v7) + Anti-Adblock Killer + Chrome Spoofing + Lazy-Load Hydrator.
//! - Chống phát hiện Adblock (Anti-Adblock Defuser): Giả lập `adsbygoogle`, `googletag`, `BlockAdBlock`,
//!   chặn `onerror` trên script mồi nhử và trả về kích thước giả cho các bait `div` (`adsbox`, `ad-banner`).
//! - Tự động kích hoạt `IntersectionObserver` và nạp ảnh `data-src` trên VnExpress.

pub const POLYFILL_SCRIPT: &str = r#"
(function () {
    'use strict';

    // ---- 1. globalThis & Chrome Environment Spoofing ----
    if (typeof globalThis === 'undefined') {
        if (typeof window !== 'undefined') window.globalThis = window;
        else if (typeof self !== 'undefined') self.globalThis = self;
    }

    if (typeof window !== 'undefined' && typeof window.chrome === 'undefined') {
        window.chrome = {
            app: { isInstalled: false },
            runtime: { id: undefined, connect: function () {}, sendMessage: function () {} },
            csi: function () { return {}; },
            loadTimes: function () { return {}; }
        };
    }

    if (typeof navigator !== 'undefined') {
        try {
            if (!navigator.vendor) {
                Object.defineProperty(navigator, 'vendor', {
                    get: function () { return 'Google Inc.'; },
                    configurable: true
                });
            }
            if (typeof navigator.userAgentData === 'undefined') {
                Object.defineProperty(navigator, 'userAgentData', {
                    get: function () {
                        return {
                            brands: [
                                { brand: 'Chromium', version: '131' },
                                { brand: 'Google Chrome', version: '131' },
                                { brand: 'Not_A Brand', version: '24' }
                            ],
                            mobile: false,
                            platform: 'Linux',
                            getHighEntropyValues: function () {
                                return Promise.resolve({
                                    architecture: 'x86',
                                    bitness: '64',
                                    model: '',
                                    platform: 'Linux',
                                    platformVersion: '6.8.0',
                                    uaFullVersion: '131.0.0.0'
                                });
                            },
                            toJSON: function () {
                                return { brands: this.brands, mobile: false, platform: 'Linux' };
                            }
                        };
                    },
                    configurable: true
                });
            }
        } catch (_) {}
    }

    // ---- 2. Anti-Adblock Defuser & Ad-Script Surrogates (Vượt qua các trình phát hiện Adblock) ----
    function isInternalPage() {
        if (typeof location === 'undefined') return true;
        var p = location.protocol || '';
        var h = location.href || '';
        return p === 'kestrel:' || p === 'about:' || p === 'data:' || h.indexOf('kestrel://') === 0;
    }

    if (typeof window !== 'undefined' && !isInternalPage()) {
        try {
            // 2.1. Giả lập các biến toàn cục mà script kiểm tra Adblock thường dò
            window.canRunAds = true;
            window.isAdBlockActive = false;
            window.adblock = false;
            window.adblockEnabled = false;
            window.google_ad_modifications = window.google_ad_modifications || {};
            window.google_jobrunner = window.google_jobrunner || {};

            if (!window.adsbygoogle || !window.adsbygoogle.loaded) {
                window.adsbygoogle = {
                    loaded: true,
                    push: function () { return 0; }
                };
            }

            // 2.2. Giả lập Google Publisher Tag (GPT / googletag)
            if (!window.googletag || !window.googletag.apiReady) {
                var noopFn = function () { return this; };
                var slotStub = {
                    addService: noopFn,
                    clearCategoryExclusions: noopFn,
                    clearTargeting: noopFn,
                    defineSizeMapping: noopFn,
                    get: function () { return null; },
                    getAdUnitPath: function () { return ''; },
                    getAttributeKeys: function () { return []; },
                    getCategoryExclusions: function () { return []; },
                    getDomId: function () { return ''; },
                    getResponseInformation: function () { return null; },
                    getSlotElementId: function () { return ''; },
                    getTargeting: function () { return []; },
                    getTargetingKeys: function () { return []; },
                    set: noopFn,
                    setCategoryExclusion: noopFn,
                    setClickUrl: noopFn,
                    setCollapseEmptyDiv: noopFn,
                    setForceSafeFrame: noopFn,
                    setSafeFrameConfig: noopFn,
                    setTargeting: noopFn
                };
                var pubadsStub = {
                    addEventListener: noopFn,
                    clear: function () {},
                    clearCategoryExclusions: noopFn,
                    clearTagForChildDirectedTreatment: noopFn,
                    clearTargeting: noopFn,
                    collapseEmptyDivs: function () {},
                    defineOutOfPagePassback: function () { return slotStub; },
                    definePassback: function () { return slotStub; },
                    disableInitialLoad: function () {},
                    display: function () {},
                    enableAsyncRendering: function () {},
                    enableLazyLoad: function () {},
                    enableSingleRequest: function () {},
                    enableSyncRendering: function () {},
                    enableVideoAds: function () {},
                    get: function () { return null; },
                    getAttributeKeys: function () { return []; },
                    getSlots: function () { return []; },
                    getTargeting: function () { return []; },
                    getTargetingKeys: function () { return []; },
                    isInitialLoadDisabled: function () { return false; },
                    refresh: function () {},
                    removeEventListener: function () {},
                    set: noopFn,
                    setCategoryExclusion: noopFn,
                    setCentering: function () {},
                    setCookieOptions: noopFn,
                    setForceSafeFrame: noopFn,
                    setLocation: noopFn,
                    setPublisherProvidedId: noopFn,
                    setRequestNonPersonalizedAds: noopFn,
                    setSafeFrameConfig: noopFn,
                    setTagForChildDirectedTreatment: noopFn,
                    setTagForUnderAgeOfConsent: noopFn,
                    setTargeting: noopFn,
                    setVideoContent: noopFn,
                    updateCorrelator: function () {}
                };
                var cmdQueue = [];
                cmdQueue.push = function (fn) {
                    if (typeof fn === 'function') {
                        try { fn(); } catch (_) {}
                    }
                    return Array.prototype.push.apply(this, arguments);
                };
                window.googletag = {
                    apiReady: true,
                    pubadsReady: true,
                    cmd: cmdQueue,
                    companionAds: function () { return { setRefreshUnfilledSlots: function () {} }; },
                    content: function () { return { setContent: function () {} }; },
                    defineOutOfPageSlot: function () { return slotStub; },
                    defineSlot: function () { return slotStub; },
                    destroySlots: function () { return true; },
                    disablePublisherConsole: function () {},
                    display: function () {},
                    enableServices: function () {},
                    getVersion: function () { return '131'; },
                    pubads: function () { return pubadsStub; },
                    setAdIframeTitle: function () {},
                    sizeMapping: function () {
                        return {
                            addSize: function () { return this; },
                            build: function () { return []; }
                        };
                    }
                };
            }

            // 2.3. Vô hiệu hóa BlockAdBlock / FuckAdBlock
            var FakeAdBlockDetector = function () {};
            FakeAdBlockDetector.prototype.on = function (detected, fn) {
                if (!detected && typeof fn === 'function') {
                    try { fn(); } catch (_) {}
                }
                return this;
            };
            FakeAdBlockDetector.prototype.onDetected = function () { return this; };
            FakeAdBlockDetector.prototype.onNotDetected = function (fn) {
                if (typeof fn === 'function') {
                    try { fn(); } catch (_) {}
                }
                return this;
            };
            FakeAdBlockDetector.prototype.check = function () { return true; };
            FakeAdBlockDetector.prototype.setOption = function () { return this; };
            var fakeInstance = new FakeAdBlockDetector();

            ['BlockAdBlock', 'FuckAdBlock', 'SniffAdBlock'].forEach(function (ctorName) {
                try {
                    Object.defineProperty(window, ctorName, {
                        get: function () { return FakeAdBlockDetector; },
                        set: function () {},
                        configurable: true
                    });
                } catch (_) {}
            });
            ['blockAdBlock', 'fuckAdBlock', 'sniffAdBlock'].forEach(function (instName) {
                try {
                    Object.defineProperty(window, instName, {
                        get: function () { return fakeInstance; },
                        set: function () {},
                        configurable: true
                    });
                } catch (_) {}
            });

            // 2.4. Chặn sự kiện error trên các thẻ <script> mồi nhử và giả lập onload thành công
            window.addEventListener('error', function (ev) {
                var t = ev && ev.target;
                if (t && t.tagName === 'SCRIPT') {
                    var src = (t.src || '').toLowerCase();
                    if (/adsbygoogle|googlesyndication|doubleclick|gpt\.js|prebid|adframe|adserver|pagead/i.test(src)) {
                        ev.stopImmediatePropagation();
                        ev.preventDefault();
                        if (typeof t.onload === 'function') {
                            try { t.onload(new Event('load')); } catch (_) {}
                        }
                    }
                }
            }, true);

            // 2.5. Đánh lừa kiểm tra kích thước các thẻ Bait/Honeypot (adsbox, pub_300x250, text-ad...)
            var BAIT_CLASS_RE = /\b(adsbox|pub_300x250|pub_728x90|text-ad|textAd|text_ad|text_ads|ad-tester|adbanner|ad-placeholder)\b/i;
            function isBaitElement(el) {
                if (!el || el.nodeType !== 1) return false;
                var cls = (typeof el.className === 'string' ? el.className : '') + ' ' + (el.id || '');
                return BAIT_CLASS_RE.test(cls) && el.children.length === 0;
            }

            if (typeof HTMLElement !== 'undefined') {
                var origOffsetHeight = Object.getOwnPropertyDescriptor(HTMLElement.prototype, 'offsetHeight');
                var origOffsetWidth = Object.getOwnPropertyDescriptor(HTMLElement.prototype, 'offsetWidth');
                if (origOffsetHeight && origOffsetHeight.get) {
                    Object.defineProperty(HTMLElement.prototype, 'offsetHeight', {
                        get: function () {
                            if (isBaitElement(this)) return 250;
                            return origOffsetHeight.get.call(this);
                        },
                        configurable: true
                    });
                }
                if (origOffsetWidth && origOffsetWidth.get) {
                    Object.defineProperty(HTMLElement.prototype, 'offsetWidth', {
                        get: function () {
                            if (isBaitElement(this)) return 300;
                            return origOffsetWidth.get.call(this);
                        },
                        configurable: true
                    });
                }
            }
        } catch (_) {}
    }

    // ---- 3. Safe Cosmetic Adblock, Anti-Adblock Modal Remover & Lazy-Load Hydrator ----
    // Không bao gồm các class mồi nhử (.adsbox, .ad-banner, .adsbygoogle rỗng) trong CSS display:none
    // để các trang test Adblock không phát hiện qua getComputedStyle()!
    var AD_SELECTORS = [
        '[id^="div-gpt-ad"]',
        '[id^="google_ads_iframe"]',
        'iframe[id^="google_ads_"]',
        'iframe[src*="doubleclick.net"]',
        'iframe[src*="googlesyndication.com"]',
        'iframe[src*="adservice.google."]',
        '#sis_banner_top',
        'iframe[src*="eclick.vn"]',
        '.item_box_ads',
        '.box_quangcao',
        '.header-banner-ads',
        'iframe[src*="admicro.vn"]',
        'iframe[src*="vcmedia.vn/ads"]',
        '.ads-zone',
        '.qc-box',
        '.box-qc',
        '.banner-qc',
        'iframe[src*="criteo."]',
        /* Anti-Adblock Popup Overlays (Funding Choices / Admiral) */
        '.fc-ab-root',
        '.fc-whitelist-root'
    ];

    var COSMETIC_ADBLOCK_CSS = AD_SELECTORS.join(',\n') + ' {\n' +
        '  display: none !important;\n' +
        '  visibility: hidden !important;\n' +
        '  height: 0 !important;\n' +
        '  min-height: 0 !important;\n' +
        '  max-height: 0 !important;\n' +
        '  margin: 0 !important;\n' +
        '  padding: 0 !important;\n' +
        '  border: none !important;\n' +
        '  overflow: hidden !important;\n' +
        '  pointer-events: none !important;\n' +
        '}\n';

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

    var combinedSelector = AD_SELECTORS.join(',');

    function hydrateLazyMedia() {
        if (typeof document === 'undefined' || !document.body) return;
        try {
            var imgs = document.querySelectorAll('img[data-src], img[data-original], img[data-lazy-src]');
            for (var i = 0; i < imgs.length; i++) {
                var img = imgs[i];
                var realSrc = img.getAttribute('data-src') || img.getAttribute('data-original') || img.getAttribute('data-lazy-src');
                if (realSrc && realSrc.indexOf('http') === 0) {
                    var cur = img.getAttribute('src') || '';
                    if (!cur || cur.indexOf('data:image') === 0 || cur.indexOf('base64') !== -1) {
                        img.setAttribute('src', realSrc);
                    }
                }
            }
            var sources = document.querySelectorAll('source[data-srcset]');
            for (var j = 0; j < sources.length; j++) {
                var srcEl = sources[j];
                var srcset = srcEl.getAttribute('data-srcset');
                if (srcset && !srcEl.getAttribute('srcset')) {
                    srcEl.setAttribute('srcset', srcset);
                }
            }
        } catch (_) {}
    }

    function scrubAdsAndHydrate() {
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

        hydrateLazyMedia();

        if (!document.body) return;
        var newlyBlocked = 0;
        try {
            var nodes = document.querySelectorAll(combinedSelector);
            for (var i = 0; i < nodes.length; i++) {
                var el = nodes[i];
                if (el.getAttribute('data-kestrel-hidden') === '1') continue;
                el.setAttribute('data-kestrel-hidden', '1');
                newlyBlocked++;
                if (el.style) {
                    el.style.setProperty('display', 'none', 'important');
                    el.style.setProperty('height', '0px', 'important');
                    el.style.setProperty('min-height', '0px', 'important');
                }
            }
            // Nếu trang khóa cuộn do popup Anti-Adblock (.fc-ab-root), mở khóa lại cuộn
            if (document.querySelector('.fc-ab-root, .fc-whitelist-root')) {
                document.body.style.setProperty('overflow', 'auto', 'important');
                if (document.documentElement) {
                    document.documentElement.style.setProperty('overflow', 'auto', 'important');
                }
            }
        } catch (_) {}

        if (newlyBlocked > 0) {
            reportCosmeticBlocked(newlyBlocked);
        }
    }

    if (!isInternalPage()) {
        scrubAdsAndHydrate();
        setTimeout(scrubAdsAndHydrate, 100);
        setTimeout(scrubAdsAndHydrate, 400);
        setTimeout(scrubAdsAndHydrate, 1200);
        setTimeout(scrubAdsAndHydrate, 2800);
        if (typeof document !== 'undefined') {
            document.addEventListener('DOMContentLoaded', scrubAdsAndHydrate, { once: true });
        }
        if (typeof window !== 'undefined') {
            window.addEventListener('load', function () {
                scrubAdsAndHydrate();
                setTimeout(scrubAdsAndHydrate, 1000);
            }, { once: true });
        }
    }

    // ---- 4. Active IntersectionObserver & ResizeObserver ----
    var DEFAULT_RECT = {
        x: 0, y: 0, width: 320, height: 200, top: 0, left: 0, right: 320, bottom: 200,
        toJSON: function () { return this; }
    };
    var DEFAULT_BOX = [{ inlineSize: 320, blockSize: 200 }];

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
                        rootBounds: DEFAULT_RECT
                    };
                });
                try { self._cb(entries, self); } catch (_) {}
                hydrateLazyMedia();
            }, 25);
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
                }, 25);
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

    // ---- 5. Modern JS Built-ins (Promise, Object, Map, Array, structuredClone, UUID) ----
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

    if (typeof queueMicrotask !== 'function') {
        globalThis.queueMicrotask = function (cb) {
            if (typeof cb !== 'function') throw new TypeError('queueMicrotask requires a function');
            Promise.resolve().then(cb).catch(function (e) {
                setTimeout(function () { throw e; }, 0);
            });
        };
    }

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

    if (typeof window !== 'undefined' && typeof window.visualViewport === 'undefined') {
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
