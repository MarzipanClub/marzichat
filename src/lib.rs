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
    cfg_if::cfg_if,
    leptos::*,
    leptos_meta::*,
    leptos_router::*,
    routes::{nav::*, stories::*, story::*, users::*},
};

pub mod api;
pub mod internationalization;
pub mod routes;
pub mod types;

include!(concat!(env!("OUT_DIR"), "/info.rs"));

/// The name of the site/product.
pub const PRODUCT_NAME: &str = "Marzichat";

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let (is_routing, set_is_routing) = create_signal(cx, false);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/marzichat.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="description" content="Leptos implementation of a HackerNews demo"/>
        // adding `set_is_routing` causes the router to wait for async data to load on new pages
        <Router set_is_routing>
            // shows a progress bar while async data are loading
            <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
            <Nav />
            <main>
                <Routes>
                    <Route path="users/:id" view=User/>
                    <Route path="stories/:id" view=Story/>
                    <Route path=":stories?" view=Stories/>
                </Routes>
            </main>
        </Router>
    }
}

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a
// lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();
            mount_to_body(move |cx| {
                view! { cx, <App/> }
            });
        }
    }
}
