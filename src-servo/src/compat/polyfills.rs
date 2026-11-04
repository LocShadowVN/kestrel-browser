// src-servo/src/compat/polyfills.rs
//! Polyfill JavaScript chuẩn hóa (v8) + Anti-Adblock Defuser + Chrome Spoofing + Lazy-Load Hydrator.
//! - Hỗ trợ `Ctrl+Click` / `Middle-Click` vào thẻ `<a>` để mở link trong tab mới (`open-new-tab`).
//! - Tôn trọng trạng thái Kestrel Shields (`window.__kestrelShieldsLevel`).
//! - Tích hợp Find-in-Page (`window.__kestrelFind`) và Page Zoom (`window.__kestrelSetZoom`).

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

    function isInternalPage() {
        if (typeof location === 'undefined') return true;
        var p = location.protocol || '';
        var h = location.href || '';
        return p === 'kestrel:' || p === 'about:' || p === 'data:' || h.indexOf('kestrel://') === 0;
    }

    function isShieldsEnabled() {
        if (typeof window !== 'undefined' && typeof window.__kestrelShieldsLevel === 'number') {
            return window.__kestrelShieldsLevel > 0;
        }
        return true;
    }

    function postToKestrel(payload) {
        try {
            if (window.servoGtk && window.servoGtk.messageHandlers &&
                window.servoGtk.messageHandlers.kestrel) {
                window.servoGtk.messageHandlers.kestrel.postMessage(JSON.stringify(payload));
                return true;
            }
        } catch (_) {}
        return false;
    }

    // ---- 1.5. Ctrl+Click & Middle-Click (Button 1) để mở link trong Tab mới ----
    if (typeof document !== 'undefined') {
        function findAnchor(el) {
            while (el && el !== document) {
                if (el.tagName && el.tagName.toUpperCase() === 'A' && el.href) {
                    return el;
                }
                el = el.parentElement;
            }
            return null;
        }

        document.addEventListener('click', function (e) {
            if (e.defaultPrevented) return;
            if (e.ctrlKey || e.metaKey) {
                var a = findAnchor(e.target);
                if (a && a.href && a.href.indexOf('javascript:') !== 0) {
                    if (postToKestrel({ action: 'open-new-tab', url: a.href })) {
                        e.preventDefault();
                        e.stopPropagation();
                    }
                }
            }
        }, true);

        document.addEventListener('auxclick', function (e) {
            if (e.defaultPrevented || e.button !== 1) return;
            var a = findAnchor(e.target);
            if (a && a.href && a.href.indexOf('javascript:') !== 0) {
                if (postToKestrel({ action: 'open-new-tab', url: a.href })) {
                    e.preventDefault();
                    e.stopPropagation();
                }
            }
        }, true);
    }

    // ---- 2. Anti-Adblock Defuser & Ad-Script Surrogates ----
    if (typeof window !== 'undefined' && !isInternalPage() && isShieldsEnabled()) {
        try {
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

            window.addEventListener('error', function (ev) {
                if (!isShieldsEnabled()) return;
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

            var BAIT_CLASS_RE = /\b(adsbox|pub_300x250|pub_728x90|text-ad|textAd|text_ad|text_ads|ad-tester|adbanner|ad-placeholder)\b/i;
            function isBaitElement(el) {
                if (!isShieldsEnabled() || !el || el.nodeType !== 1) return false;
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
        'iframe[src*="taboola.com"]',
        'iframe[src*="outbrain.com"]',
        'iframe[src*="mgid.com"]',
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
        if (count <= 0 || !isShieldsEnabled()) return;
        postToKestrel({
            action: 'adblock-cosmetic-hit',
            count: count
        });
    }

    var combinedSelector = AD_SELECTORS.join(',');

    function hydrateLazyMedia() {
        if (typeof document === 'undefined' || !document.body) return;
        try {
            var imgs = document.querySelectorAll('img[data-src], img[data-original], img[data-lazy-src]');
            for (var i = 0; i < imgs.length; i++) {
                var img = imgs[i];
                var realSrc = img.getAttribute('data-src') || img.getAttribute('data-original') || img.getAttribute('data-lazy-src');
                if (realSrc && (realSrc.indexOf('http') === 0 || realSrc.indexOf('//') === 0)) {
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

        hydrateLazyMedia();

        if (!isShieldsEnabled()) {
            var existingStyle = document.getElementById('kestrel-cosmetic-adblock');
            if (existingStyle && existingStyle.parentNode) {
                existingStyle.parentNode.removeChild(existingStyle);
            }
            return;
        }

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
                if (el.style) {
                    el.style.setProperty('display', 'none', 'important');
                    el.style.setProperty('height', '0px', 'important');
                    el.style.setProperty('min-height', '0px', 'important');
                }
            }
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

    // ---- 4. Built-in Find-in-Page (`window.__kestrelFind`) & Page Zoom (`window.__kestrelSetZoom`) ----
    if (typeof window !== 'undefined') {
        window.__kestrelSetZoom = function (factor) {
            var z = Number(factor) || 1;
            if (document && document.documentElement) {
                document.documentElement.style.zoom = String(z);
            }
        };

        window.__kestrelFind = function (query, backwards) {
            if (!query) {
                if (window.getSelection) {
                    var sel = window.getSelection();
                    if (sel && sel.removeAllRanges) sel.removeAllRanges();
                }
                return false;
            }
            if (typeof window.find === 'function') {
                return window.find(String(query), false, !!backwards, true, false, true, false);
            }
            return false;
        };
    }

    // ---- 5. Geometry-Aware IntersectionObserver & ResizeObserver ----
    var DEFAULT_RECT = {
        x: 0, y: 0, width: 320, height: 200, top: 0, left: 0, right: 320, bottom: 200,
        toJSON: function () { return this; }
    };
    var DEFAULT_BOX = [{ inlineSize: 320, blockSize: 200 }];

    if (typeof globalThis.IntersectionObserver === 'undefined') {
        var activeIOInstances = [];

        function computeElementRect(el) {
            try {
                if (el && typeof el.getBoundingClientRect === 'function') {
                    var r = el.getBoundingClientRect();
                    if (r && (r.width > 0 || r.height > 0 || r.top !== 0 || r.bottom !== 0)) {
                        return r;
                    }
                }
            } catch (_) {}
            return DEFAULT_RECT;
        }

        function checkAllIOInstances() {
            for (var i = 0; i < activeIOInstances.length; i++) {
                activeIOInstances[i]._evaluate();
            }
        }

        if (typeof window !== 'undefined') {
            window.addEventListener('scroll', checkAllIOInstances, { passive: true });
            window.addEventListener('resize', checkAllIOInstances, { passive: true });
        }

        globalThis.IntersectionObserver = function IntersectionObserver(callback, options) {
            this._cb = callback;
            this._targets = [];
            this._state = new WeakMap();
            this._scheduled = false;
            this.root = (options && options.root) || null;
            this.rootMargin = (options && options.rootMargin) || '0px';
            this.thresholds = (options && Array.isArray(options.threshold))
                ? options.threshold
                : [(options && typeof options.threshold === 'number') ? options.threshold : 0];
            activeIOInstances.push(this);
        };

        globalThis.IntersectionObserver.prototype._schedule = function () {
            if (this._scheduled) return;
            this._scheduled = true;
            var self = this;
            setTimeout(function () {
                self._scheduled = false;
                self._evaluate();
            }, 25);
        };

        globalThis.IntersectionObserver.prototype._evaluate = function () {
            if (this._targets.length === 0 || typeof this._cb !== 'function') return;
            var vw = (typeof window !== 'undefined' && window.innerWidth) || 1280;
            var vh = (typeof window !== 'undefined' && window.innerHeight) || 900;
            var preloadMargin = 600;
            var now = (typeof performance !== 'undefined' && performance.now) ? performance.now() : Date.now();
            var rootBounds = {
                x: 0, y: 0, top: 0, left: 0, width: vw, height: vh, right: vw, bottom: vh
            };
            var changedEntries = [];

            for (var i = 0; i < this._targets.length; i++) {
                var el = this._targets[i];
                var rect = computeElementRect(el);
                var isIntersecting = (
                    rect.bottom >= -preloadMargin &&
                    rect.top <= (vh + preloadMargin) &&
                    rect.right >= -preloadMargin &&
                    rect.left <= (vw + preloadMargin)
                );
                var prev = this._state.get(el);
                if (prev !== isIntersecting) {
                    this._state.set(el, isIntersecting);
                    changedEntries.push({
                        time: now,
                        target: el,
                        isIntersecting: isIntersecting,
                        intersectionRatio: isIntersecting ? 1 : 0,
                        boundingClientRect: rect,
                        intersectionRect: isIntersecting ? rect : {
                            x: 0, y: 0, top: 0, left: 0, width: 0, height: 0, right: 0, bottom: 0
                        },
                        rootBounds: rootBounds
                    });
                }
            }

            if (changedEntries.length > 0) {
                try { this._cb(changedEntries, this); } catch (_) {}
                hydrateLazyMedia();
            }
        };

        globalThis.IntersectionObserver.prototype.observe = function (target) {
            if (!target || typeof this._cb !== 'function') return;
            if (this._targets.indexOf(target) === -1) {
                this._targets.push(target);
                this._schedule();
            }
        };

        globalThis.IntersectionObserver.prototype.unobserve = function (target) {
            var idx = this._targets.indexOf(target);
            if (idx !== -1) this._targets.splice(idx, 1);
            this._state.delete(target);
        };

        globalThis.IntersectionObserver.prototype.disconnect = function () {
            this._targets.length = 0;
            var idx = activeIOInstances.indexOf(this);
            if (idx !== -1) activeIOInstances.splice(idx, 1);
        };

        globalThis.IntersectionObserver.prototype.takeRecords = function () { return []; };
    }

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

    if (typeof globalThis.PerformanceObserver === 'undefined') {
        globalThis.PerformanceObserver = function PerformanceObserver(callback) {
            this._cb = callback;
        };
        globalThis.PerformanceObserver.supportedEntryTypes = [
            'navigation', 'resource', 'mark', 'measure', 'paint', 'longtask', 'largest-contentful-paint', 'layout-shift'
        ];
        globalThis.PerformanceObserver.prototype.observe = function () {};
        globalThis.PerformanceObserver.prototype.disconnect = function () {};
        globalThis.PerformanceObserver.prototype.takeRecords = function () { return []; };
    }

    // ---- 6. Modern JS Built-ins (ES2022–ES2024: Promise, Array, Set, Object, Map, URL, Crypto) ----
    if (typeof globalThis.AggregateError === 'undefined') {
        globalThis.AggregateError = function AggregateError(errors, message) {
            var err = new Error(message || '');
            err.name = 'AggregateError';
            err.errors = Array.isArray(errors) ? errors : Array.from(errors || []);
            return err;
        };
        globalThis.AggregateError.prototype = Object.create(Error.prototype);
    }

    if (typeof Promise !== 'undefined') {
        if (typeof Promise.withResolvers !== 'function') {
            Object.defineProperty(Promise, 'withResolvers', {
                value: function () {
                    var resolve, reject;
                    var promise = new Promise(function (res, rej) { resolve = res; reject = rej; });
                    return { promise: promise, resolve: resolve, reject: reject };
                },
                writable: true, configurable: true
            });
        }
        if (typeof Promise.allSettled !== 'function') {
            Object.defineProperty(Promise, 'allSettled', {
                value: function (iterable) {
                    var arr = Array.from(iterable || []).map(function (p) {
                        return Promise.resolve(p).then(
                            function (value) { return { status: 'fulfilled', value: value }; },
                            function (reason) { return { status: 'rejected', reason: reason }; }
                        );
                    });
                    return Promise.all(arr);
                },
                writable: true, configurable: true
            });
        }
        if (typeof Promise.any !== 'function') {
            Object.defineProperty(Promise, 'any', {
                value: function (iterable) {
                    var items = Array.from(iterable || []);
                    if (items.length === 0) {
                        return Promise.reject(new globalThis.AggregateError([], 'All promises were rejected'));
                    }
                    return new Promise(function (resolve, reject) {
                        var errors = new Array(items.length);
                        var rejectedCount = 0;
                        items.forEach(function (item, idx) {
                            Promise.resolve(item).then(resolve, function (err) {
                                errors[idx] = err;
                                rejectedCount++;
                                if (rejectedCount === items.length) {
                                    reject(new globalThis.AggregateError(errors, 'All promises were rejected'));
                                }
                            });
                        });
                    });
                },
                writable: true, configurable: true
            });
        }
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

    if (typeof Object.fromEntries !== 'function') {
        Object.defineProperty(Object, 'fromEntries', {
            value: function (iter) {
                var obj = {};
                for (var pair of iter) {
                    if (pair && typeof pair === 'object') {
                        obj[pair[0]] = pair[1];
                    }
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
                    if (!list) {
                        list = [];
                        map.set(key, list);
                    }
                    list.push(item);
                }
                return map;
            },
            writable: true, configurable: true
        });
    }

    if (typeof Array !== 'undefined') {
        if (typeof Array.prototype.at !== 'function') {
            Object.defineProperty(Array.prototype, 'at', {
                value: function (n) {
                    n = Math.trunc(n) || 0;
                    if (n < 0) n += this.length;
                    if (n < 0 || n >= this.length) return undefined;
                    return this[n];
                },
                writable: true, configurable: true
            });
        }
        if (typeof Array.prototype.findLast !== 'function') {
            Object.defineProperty(Array.prototype, 'findLast', {
                value: function (predicate, thisArg) {
                    for (var i = this.length - 1; i >= 0; i--) {
                        if (predicate.call(thisArg, this[i], i, this)) return this[i];
                    }
                    return undefined;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Array.prototype.findLastIndex !== 'function') {
            Object.defineProperty(Array.prototype, 'findLastIndex', {
                value: function (predicate, thisArg) {
                    for (var i = this.length - 1; i >= 0; i--) {
                        if (predicate.call(thisArg, this[i], i, this)) return i;
                    }
                    return -1;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Array.prototype.toSorted !== 'function') {
            Object.defineProperty(Array.prototype, 'toSorted', {
                value: function (compareFn) {
                    return Array.from(this).sort(compareFn);
                },
                writable: true, configurable: true
            });
        }
        if (typeof Array.prototype.toReversed !== 'function') {
            Object.defineProperty(Array.prototype, 'toReversed', {
                value: function () {
                    return Array.from(this).reverse();
                },
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
                    var i = Math.trunc(index) || 0;
                    if (i < 0) i += copy.length;
                    if (i < 0 || i >= copy.length) throw new RangeError('Invalid index');
                    copy[i] = value;
                    return copy;
                },
                writable: true, configurable: true
            });
        }
    }

    if (typeof String !== 'undefined' && typeof String.prototype.at !== 'function') {
        Object.defineProperty(String.prototype, 'at', {
            value: function (n) {
                n = Math.trunc(n) || 0;
                if (n < 0) n += this.length;
                if (n < 0 || n >= this.length) return undefined;
                return this.charAt(n);
            },
            writable: true, configurable: true
        });
    }

    if (typeof Set !== 'undefined') {
        if (typeof Set.prototype.union !== 'function') {
            Object.defineProperty(Set.prototype, 'union', {
                value: function (other) {
                    var res = new Set(this);
                    if (other && typeof other.forEach === 'function') {
                        other.forEach(function (v) { res.add(v); });
                    }
                    return res;
                },
                writable: true, configurable: true
            });
        }
        if (typeof Set.prototype.intersection !== 'function') {
            Object.defineProperty(Set.prototype, 'intersection', {
                value: function (other) {
                    var res = new Set();
                    if (other && typeof other.has === 'function') {
                        this.forEach(function (v) {
                            if (other.has(v)) res.add(v);
                        });
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
                    if (other && typeof other.forEach === 'function') {
                        other.forEach(function (v) { res.delete(v); });
                    }
                    return res;
                },
                writable: true, configurable: true
            });
        }
    }

    if (typeof URL !== 'undefined' && typeof URL.canParse !== 'function') {
        Object.defineProperty(URL, 'canParse', {
            value: function (url, base) {
                try {
                    new URL(url, base);
                    return true;
                } catch (_) {
                    return false;
                }
            },
            writable: true, configurable: true
        });
    }

    if (typeof crypto !== 'undefined' && typeof crypto.randomUUID !== 'function') {
        try {
            crypto.randomUUID = function () {
                return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
                    var r = (Math.random() * 16) | 0;
                    var v = c === 'x' ? r : (r & 0x3) | 0x8;
                    return v.toString(16);
                });
            };
        } catch (_) {}
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

    if (typeof window !== 'undefined') {
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

        if (typeof window.matchMedia !== 'function') {
            window.matchMedia = function (query) {
                var q = String(query || '').toLowerCase();
                var matches = false;
                if (q === '' || q === 'all' || q === 'screen') matches = true;
                else if (q.indexOf('prefers-color-scheme: dark') !== -1) matches = true;
                return {
                    matches: matches,
                    media: String(query || ''),
                    onchange: null,
                    addListener: function () {},
                    removeListener: function () {},
                    addEventListener: function () {},
                    removeEventListener: function () {},
                    dispatchEvent: function () { return false; }
                };
            };
        }

        if (typeof window.CSS === 'undefined') {
            window.CSS = {};
        }
        if (typeof window.CSS.supports !== 'function') {
            window.CSS.supports = function () { return true; };
        }
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
