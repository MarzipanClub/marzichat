// unstable feature https://github.com/rust-lang/rust/issues/70142
// enables `Result::flatten()` method
#![feature(result_flattening)]

pub mod app;

/// The name of the CSS file.
pub const CSS_FILE_NAME: &str = env!("CSS_FILE_NAME");

/// The path segment for static assets.
pub const ASSETS_PATH: &str = "assets";
