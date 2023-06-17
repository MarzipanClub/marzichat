//! A CLI utility for interacting with the datastore

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
    clap::{Parser, Subcommand},
    std::{path::PathBuf, sync::OnceLock},
};

#[derive(Parser)]
#[command(author, version, about, long_about = "CLI for datastore operations")]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "CONFIG")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a value from the datastore
    Get {
        /// the key value to get
        key: String,
    },
    /// Set a value in the datastore
    Set {
        /// the key value to set, will error if key is taken
        key: String,
        /// lists test values
        value: String,
        /// forces setting the value if the key is taken
        #[arg(short, long)]
        force: bool,
    },
}

static DEBUG_LEVEL: OnceLock<u8> = OnceLock::new();

pub fn debug_level() -> u8 {
    *DEBUG_LEVEL.get().expect("debug level not set")
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences

    DEBUG_LEVEL
        .set(cli.debug)
        .expect("unable to set debug level");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Get { key } => {
            println!("Getting value for key: {key}");
        }
        Commands::Set { key, value, force } => {
            println!("Setting value for key: {key} to {value} with force: {force}");
        }
    }
}
