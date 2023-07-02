//! This module is run at compile time to gather build info.

use {
    anyhow::Result,
    common::api::{PING_INTERVAL, PONG_TIMEOUT},
    std::{env, fs::File, io::Write, path::Path},
};

/// Gather compile-time build info.
fn main() -> Result<()> {
    assert!(
        PONG_TIMEOUT > PING_INTERVAL,
        "PONG_TIMEOUT {PONG_TIMEOUT:?} must be greater than PING_INTERVAL {PING_INTERVAL:?}"
    );

    let rustc_version = {
        let mut version = String::from_utf8(
            std::process::Command::new("rustc")
                .arg("-V")
                .output()?
                .stdout,
        )?;
        version.pop(); // remove newline
        version
    };

    let git_sha = git2::Repository::discover(env::var("CARGO_MANIFEST_DIR")?)?
        .head()?
        .peel_to_commit()?
        .id()
        .to_string();

    let git_short_sha = &git_sha[0..=6];

    let mut buf = Vec::new();

    // list of environmental variables that Cargo sets for crates:
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    for (name, value) in [
        ("VERSION", env::var("CARGO_PKG_VERSION")?),
        ("PROFILE", env::var("PROFILE")?),
        ("COMPILER", rustc_version),
        ("GIT_SHA", git_sha.to_owned()),
        ("GIT_SHORT_SHA", git_short_sha.to_string()),
        ("BUILD_TIME", chrono::Utc::now().to_rfc3339()),
    ] {
        writeln!(&mut buf, "pub const {name}: &str = \"{value}\";")?;
    }

    File::create(Path::new(&env::var("OUT_DIR")?).join("info.rs"))?.write_all(&buf)?;
    Ok(())
}
