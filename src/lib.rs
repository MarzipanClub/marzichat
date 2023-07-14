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
    crate::{internationalization::Language, routes::*, types::*},
    components::*,
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

/// Company name.
pub const COMPANY_NAME: &str = "Marzipan Club, LLC";

/// The site url.
pub const SITE_URL: &str = "marzichat.com";

/// Customer support email address.
pub const SUPPORT_EMAIL: &str = "hello@marzipan.club";

/// The site-root relative folder where all compiled output is written to by
/// leptos.
pub const OUT_DIR: &str = "/pkg";

/// Copyright notice.
pub const COPYRIGHT: &str = "© 2021 Marzichat";

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! { cx,
        <Stylesheet href=formatcp!("{OUT_DIR}/{}.css", env!("CARGO_PKG_NAME"))/>
        <Meta name="description" content="The best forum on the internet."/>
        <Router>
            <Nav/>
            <div class="container-xl p-3 pt-8">
                <Routes>
                    <Route path=HOME view=Stories/>
                    <Route path="users/:id" view=User/>
                    <Route path="stories/:id" view=Story/>

                    <Route path=SIGNIN view=Signin/>
                    <Route path=SIGNUP view=Signup/>

                    <Route path=ABOUT view=NotFound/> // TODO: build about page

                    <Route path=HELP_AND_SAFETY view=NotFound/> // TODO: build HelpAndSafety page
                    <Route path="/help" view=|cx| view! { cx, <Redirect path=HELP_AND_SAFETY/> }/>
                    <Route path="/safety" view=|cx| view! { cx, <Redirect path=HELP_AND_SAFETY/> }/>

                    <Route path=PRIVACY_POLICY view=PrivacyPolicy/>
                    <Route path="/privacy" view=|cx| view! { cx, <Redirect path=PRIVACY_POLICY/> }/>

                    <Route path=TERMS_AND_CONDITIONS view=TermsAndConditions/>
                    <Route path="/terms" view=|cx| view! { cx, <Redirect path=TERMS_AND_CONDITIONS/> }/>

                    <Route path="*" view=NotFound/>
                </Routes>
            </div>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    wasm_bindgen::UnwrapThrowExt::unwrap_throw(console_log::init_with_level(log::Level::Debug));
    mount_to_body(App);
}

/// Returns a summary of the build info in plain text format.
pub fn summary() -> String {
    let build_time = DateTime::from(
        chrono::DateTime::parse_from_rfc3339(BUILD_TIME).expect("error parsing build time"),
    );
    let built_on = build_time.format("%c %Z").to_string();
    let ago = datetime::ago(&build_time, Language::English);

    let logo = indoc::indoc! {r#"
         __  __                _      _           _
        |  \/  |              (_)    | |         | |
        | \  / | __ _ _ __ _____  ___| |__   __ _| |_
        | |\/| |/ _` | '__|_  / |/ __| '_ \ / _` | __|
        | |  | | (_| | |   / /| | (__| | | | (_| | |_
        |_|  |_|\__,_|_|  /___|_|\___|_| |_|\__,_|\__|

    "#};
    let copyright = copyright();
    indoc::formatdoc! {"
        {logo}
        {PRODUCT_NAME} {VERSION} ({GIT_SHORT_SHA})
        Built on {built_on} ({ago}).
        {GIT_SHA}
        {COMPILER}
        {copyright}

        Made with ❤️ in Seattle by Jesus Guzman, Jr.
    "}
}

/// Returns the copyright statement.
pub fn copyright() -> &'static str {
    static COPYRIGHT: OnceLock<String> = OnceLock::new();
    COPYRIGHT.get_or_init(|| {
        format!(
            "© {} {COMPANY_NAME}",
            chrono::Utc::now().date_naive().format("%Y").to_string()
        )
    })
}

/// Listen for clicks outside of an element. Useful for modals or dropdowns.
#[allow(unused_variables)]
pub fn on_click_outside<El, T, F>(cx: Scope, target: El, handler: F)
where
    El: Clone,
    (Scope, El): Into<leptos_use::core::ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
    F: FnMut(web_sys::Event) + Clone + 'static,
{
    #[cfg(feature = "hydrate")]
    drop(leptos_use::on_click_outside(cx, target, handler));
}

/// Scroll to the top of the page on first render.
pub fn scroll_to_top() {
    #[cfg(feature = "hydrate")]
    window().scroll_to_with_x_and_y(0.0, 0.0);
}
