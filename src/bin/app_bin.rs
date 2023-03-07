use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;

// Service worker registration
fn register_service_worker(service_worker_url: &str) {
    let window = web_sys::window().expect("no global window exists");
    
    if JsValue::from_str(&"serviceWorker").js_in(&window.navigator()) {
        web_sys::console::log_2(&"Register Service Worker : ".into(),&service_worker_url.into());

        // Set options for register
        let mut options = web_sys::RegistrationOptions::new();
        options.scope("/");
        
        // Register
        let promise = window.navigator().service_worker().register_with_options(service_worker_url, &options);
        let _registration = JsFuture::from(promise);
    } else {
        web_sys::console::log_1(&"Navigator without serviceWorker".into()); 
    }
}

// Main function for the application init (main rendering)
fn main() {
    register_service_worker("/service_worker.js");
    yew::Renderer::<positive_life::app::App>::new().render();
}