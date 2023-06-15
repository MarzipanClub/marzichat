#![feature(result_flattening)]

use {marzichat::App, wasm_bindgen::prelude::wasm_bindgen};

#[wasm_bindgen]
pub fn hydrate() {
    wasm_log::init(wasm_log::Config::default());
    console_error_panic_hook::set_once();
    leptos::mount_to_body(move |cx| {
        leptos::view! { cx, <App/> }
    });
}
