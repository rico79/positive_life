use wasm_bindgen::prelude::*;
use web_sys::*;
use gloo::events::EventListener;

// Main of the service worker logic (called by the js file)
fn main() {
    web_sys::console::log_1(&"Service worker called !!'".into());
    
    // Get worker global scope
    let self_scope = js_sys::global().dyn_into::<WorkerGlobalScope>().expect("Service Worker should have a global scope");

    // Event listener on install
    let _install_event = EventListener::new(&self_scope, "install", move |_event| {
        web_sys::console::log_1(&"Service worker install event !!'".into());
    });
}