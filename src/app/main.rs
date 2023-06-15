// no main function needed for leptos
#![no_main]

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use core::app::App;

    wasm_log::init(wasm_log::Config::default());
    console_error_panic_hook::set_once();
    leptos::mount_to_body(move |cx| {
        leptos::view! { cx, <App/> }
    });
}
