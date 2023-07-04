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
    crate::pages::{Home, Signup},
    common::{
        routes::{PageRoutes, ASSETS_PATH, CSS_FILE_NAME},
        PRODUCT_NAME,
    },
    const_format::formatcp,
    leptos::*,
    leptos_meta::*,
    leptos_router::*,
};

pub mod pages;

mod components;
mod websocket;

/// The app UI entry point.
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    // TODO: add open graph meta tags

    websocket::provide(cx);

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
