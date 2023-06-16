// unstable feature: https://github.com/rust-lang/rust/issues/70142
// Enables `Result::flatten()` method.
#![feature(result_flattening)]

//! App library code.

// rustc lints
// https://doc.rust-lang.org/rustc/lints/index.html
// note that unused_crate_dependencies causes false positives
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

pub mod ui;

/// The name of the CSS file.
pub const CSS_FILE_NAME: &str = env!("CSS_FILE_NAME");

/// The path segment for static assets.
pub const ASSETS_PATH: &str = "assets";
