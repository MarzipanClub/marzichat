//! Compile-time build information.

use common::{internationalization::Language, types::DateTime, PRODUCT_NAME};

include!(concat!(env!("OUT_DIR"), "/info.rs"));

/// Returns a summary of the build info in plain text format.
pub fn summary() -> String {
    let build_time = common::types::datetime::ago(
        &DateTime::from(chrono::DateTime::parse_from_rfc3339(BUILD_TIME).unwrap()),
        Language::English,
    );
    format!(
            "{PRODUCT_NAME} v{VERSION} ({GIT_SHORT_SHA})\nBuilt {build_time}.\n\n{BUILD_TIME}\n{GIT_SHA}\n{COMPILER}"
        )
}
