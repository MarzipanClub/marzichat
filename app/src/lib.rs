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
    crate::pages::Signup, common::internationalization::Translations, const_format::formatcp,
    leptos::*, leptos_meta::*, leptos_router::*,
};

pub mod pages;

mod context;

mod components;

/// The official name of the product.
pub const PRODUCT_NAME: &str = "Marzichat";

/// The name of the CSS file.
pub const CSS_FILE_NAME: &str = env!("CSS_FILE_NAME");

/// The path segment for static assets.
pub const ASSETS_PATH: &str = "assets";

/// The name of the language local storage key.
const LANGUAGE_KEY: &str = "language";

/// The app UI entry point.
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    // let language = if let Ok(Some(storage)) =
    // gloo::storage::LocalStorage::get(LANGUAGE_KEY) {     storage
    //         .get_item(LANGUAGE_KEY)
    //         .ok()
    //         .flatten()
    //         .and_then(|language| Language::parse_from_bcp47_tag(&language).ok())
    //         .unwrap_or_default()
    // } else {
    //     Default::default()
    // };

    // let translations = Translations::for_language(language);
    // provide_context(cx, PreferredTranslations(create_signal(cx,
    // translations).0));

    // TODO: add open graph meta tags

    view! { cx,
        <Stylesheet href=formatcp!("/{ASSETS_PATH}/{CSS_FILE_NAME}")/>
        <Title text={PRODUCT_NAME}/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Signup/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Copy, Clone)]
struct PreferredTranslations(ReadSignal<Translations>);

// /// Returns the preferred language of the user or the default language if
// /// obtaining the preferred language fails.
// fn preferred_language() -> Language {
//     if let Ok(Some(storage)) = window().local_storage() {
//         storage
//             .get_item(LANGUAGE_KEY)
//             .ok()
//             .flatten()
//             .and_then(|language|
// Language::parse_from_bcp47_tag(&language).ok())
// .unwrap_or_default()     } else {
//         Default::default()
//     }
// }

// /// Gets the preferred language of the user.
// fn get_translations(cx: Scope) -> Translations {
//     use_context::<PreferredTranslations>(cx)
//         .expect("preferred translations not provided")
//         .0()
// }
