export LOG := "backend=trace,actix_server=warn,hyper=warn,reqwest=warn,sqlx=info,debug"

# list all recipes
default:
    just --list

# check the code for compile errors
check:
    cargo check-all-features
    cargo clippy

# command for rust analyzer check
rust-analyzer-check:
    cargo check-all-features --message-format=json-diagnostic-rendered-ansi

# run watching for changes
watch:
    cargo leptos watch