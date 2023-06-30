//! Shared library code.

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

pub mod build;
pub mod internationalization;
pub mod routes;
pub mod types;

/// The name of the site/product.
pub const PRODUCT_NAME: &str = "Marzichat";
