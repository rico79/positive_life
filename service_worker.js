/*self.importScripts('service_worker_bin.js');
wasm_bindgen.init(location.origin + "/service_worker_bin_bg.wasm");*/

/* cache ressources to be get offline */
const addResourcesToCache = async (resources) => {
    console.log("Adding ressources to cache");
    const cache = await caches.open("v1");
    await cache.addAll(resources);
};

/* Get from cache and if not, fetch from network */
const cacheFirst = async (request) => {
    const responseFromCache = await caches.match(request);
    if (responseFromCache) {
        console.log("Returning from cache : ", request.url);
        return responseFromCache;
    }
    console.log("Returning from request : ", request.url);
    return fetch(request);
};

/* Install listener */
self.addEventListener("install", (event) => {
    event.waitUntil(
        addResourcesToCache([
            "/",
            "/index.html",
            "/index.css",
            "/app_bin.js",
            "/app_bin_bg.wasm",
            "/icons/lotus-120.png",
            "/icons/lotus-150.png",
            "/icons/lotus-256.png",
        ])
    );
});

/* fetch listener */
self.addEventListener("fetch", (event) => {
    event.respondWith(cacheFirst(event.request));
});
  