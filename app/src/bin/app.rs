// No main function needed for app. The browser will instead call the hydrate
// function.
#![no_main]

//! The main entry point for the app.

// rustc lints
// https://doc.rust-lang.org/rustc/lints/index.html
#![forbid(unsafe_code, let_underscore_lock)]
#![deny(unused_extern_crates)]
#![warn(
    future_incompatible,
    let_underscore_drop,
    rust_2018_idioms,
    single_use_lifetimes,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_tuple_struct_fields,
    variant_size_differences
)]

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::ui::Main;

    wasm_log::init(wasm_log::Config::default());
    console_error_panic_hook::set_once();
    leptos::mount_to_body(move |cx| {
        leptos::view! { cx, <Main/> }
    });
}
