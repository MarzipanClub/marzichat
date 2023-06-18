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
    anyhow::Result,
    clap::{Parser, Subcommand},
    std::{net::SocketAddr, sync::OnceLock},
    tarpc::context,
};

#[derive(Parser)]
#[command(author, version, about, long_about = "CLI for datastore operations")]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Get a value from the datastore
    Get {
        /// the name of the table
        table: String,
        /// the key value to get
        key: String,
    },
    /// Set a value in the datastore
    Set {
        /// the name of the table
        table: String,
        /// the key value to set, will error if key is taken
        key: String,
        /// lists test values
        value: String,
        /// forces setting the value if the key is taken
        #[arg(short, long)]
        force: bool,
    },
    /// Delete a value from the datastore
    Delete {
        /// the name of the table
        table: String,
        /// the key value to delete
        key: String,
    },
    /// List all tables in the datastore
    List,
}

static DEBUG_LEVEL: OnceLock<u8> = OnceLock::new();

pub fn debug_level() -> u8 {
    *DEBUG_LEVEL.get().expect("debug level not set")
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let socket_address = common::utils::config::env_var::<SocketAddr>("DATASTORE_SOCKET_ADDRESS")?;

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences

    DEBUG_LEVEL
        .set(cli.debug)
        .expect("unable to set debug level");

    if debug_level() > 0 {
        println!("connecting to socket address: {}", socket_address);
    }
    let client = datastore::client(socket_address).await?;
    let context: context::Context = context::current();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Command::Get { table, key } => {
            if debug_level() > 0 {
                println!("getting value for key: {key}");
            }
            match client.get(context, table, key).await?? {
                Some(value) => println!("{value}"),
                None => eprintln!("no value was set"),
            }
        }
        Command::Set {
            table,
            key,
            value,
            force,
        } => {
            if debug_level() > 0 {
                println!("setting value for key: {key} to {value} with force: {force}");
            }

            if client
                .get(context, table.clone(), key.clone())
                .await??
                .is_some()
            {
                if force {
                    client.set(context, table, key, value).await??;
                } else {
                    eprintln!("key: {key} already exists, use --force to overwrite");
                }
            } else {
                client.set(context, table, key, value).await??;
            }
        }
        Command::Delete { table, key } => {
            if debug_level() > 0 {
                println!("deleting value for key: {key}");
            }
            client.delete(context, table, key).await??;
        }
        Command::List => {
            if debug_level() > 0 {
                println!("listing tables");
            }
            match client.list(context).await?? {
                Some(tables) => {
                    for table in tables {
                        println!("{}", table);
                    }
                }
                None => eprintln!("no tables found"),
            }
        }
    }

    Ok(())
}
