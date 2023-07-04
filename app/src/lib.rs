// unstable feature: https://github.com/rust-lang/rust/issues/70142
// Enables `Result::flatten()` method.
#![feature(result_flattening)]

//! App library code.

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

use {
    crate::{
        pages::{Home, Signup},
        stream::provide_connection,
    },
    common::{
        routes::{PageRoutes, ASSETS_PATH, CSS_FILE_NAME, WEBSOCKET_URL},
        PRODUCT_NAME,
    },
    const_format::formatcp,
    leptos::*,
    leptos_meta::*,
    leptos_router::*,
    wasm_bindgen::UnwrapThrowExt,
};

pub mod pages;

mod components;
mod stream;

/// The app UI entry point.
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    #[cfg(feature = "hydrate")]
    let websocket = web_sys::WebSocket::new(WEBSOCKET_URL).unwrap_throw();
    #[cfg(feature = "hydrate")]
    provide_context(cx, websocket);

    provide_connection(cx);

    // TODO: add open graph meta tags

    view! { cx,
        <Stylesheet href=formatcp!("/{ASSETS_PATH}/{CSS_FILE_NAME}")/>
        <Title text={PRODUCT_NAME}/>

        <Router>
            <main>
                <Routes>
                    <Route path={PageRoutes::Home} view=|cx| view! { cx, <Home/> }/>
                    <Route path={PageRoutes::Signup} view=|cx| view! { cx, <Signup/> }/>
                </Routes>
            </main>
        </Router>
    }
}
