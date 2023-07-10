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
        internationalization::{Language, Translations},
        routes::Routes,
        types::*,
    },
    const_format::formatcp,
    leptos::*,
    leptos_meta::*,
    leptos_router::*,
    routes::{
        not_found::*, privacy::*, signin::*, signup::*, stories::*, story::*, terms::*, users::*,
    },
    std::sync::OnceLock,
};

pub mod api;
pub mod components;
pub mod internationalization;
pub mod routes;
pub mod types;

include!(concat!(env!("OUT_DIR"), "/info.rs"));

/// The name of the site/product.
pub const PRODUCT_NAME: &str = "Marzichat";

/// Customer support email address.
pub const SUPPORT_EMAIL: &str = "hello@marzipan.club";

/// The site-root relative folder where all compiled output is written to by
/// leptos.
pub const OUT_DIR: &str = "/pkg";

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let t = Translations::default();

    view! { cx,
        <Stylesheet href=formatcp!("{OUT_DIR}/{}.css", env!("CARGO_PKG_NAME"))/>
        <Meta name="description" content="The best forum on the internet."/>

        <div style="max-width: 1280px; min-height: 100" class="mx-auto mb-0 py-1 px-2">
        <Router>
             <nav class="UnderlineNav" aria-label="nav bar">
                <div class="UnderlineNav-body">
                    <a class="UnderlineNav-item app-link" href="#home" aria-current="page">{"Home"}</a>
                    <a class="UnderlineNav-item app-link" href="#about">{"About"}</a>
                    <a class="UnderlineNav-item app-link" href="#newsletter">{"Newsletter"}</a>
                </div>
                <div class="UnderlineNav-actions">
                    <A href=Routes::Signin class="btn btn-sm mx-2">{t.sign_in()}</A>
                    <A href=Routes::Signup class="btn btn-sm btn-primary">{t.sign_up()}</A>
                </div>
            </nav>
            <Routes>
                <Route path=Routes::Home view=Stories/>
                <Route path="users/:id" view=User/>
                <Route path="stories/:id" view=Story/>
                <Route path=Routes::Signin view=Signin/>
                <Route path=Routes::Signup view=Signup/>
                <Route path=Routes::PrivacyPolicy view=PrivacyPolicy/>
                <Route path=Routes::TermsAndConditions view=TermsAndConditions/>
                <Route path="*" view=NotFound/>
            </Routes>
        </Router>
        </div>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}

/// Returns a summary of the build info in plain text format.
pub fn summary() -> String {
    let build_time = datetime::ago(
        &DateTime::from(
            chrono::DateTime::parse_from_rfc3339(BUILD_TIME).expect("error parsing build time"),
        ),
        Language::English,
    );
    format!(
            "{PRODUCT_NAME} v{VERSION} ({GIT_SHORT_SHA})\nBuilt {build_time}.\n\n{BUILD_TIME}\n{GIT_SHA}\n{COMPILER}"
        )
}

/// Returns the current year as a static string.
pub fn current_year() -> &'static str {
    static YEAR: OnceLock<String> = OnceLock::new();
    YEAR.get_or_init(|| chrono::Utc::now().date_naive().format("%Y").to_string())
}
