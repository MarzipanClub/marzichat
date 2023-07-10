export LOG := "marzichat=trace,actix_files=trace,hyper=warn,reqwest=warn,sqlx=info,debug"
export DATABASE_URL := "postgresql://marzichat@127.0.0.1/marzichat"
export REPLENISH_RATE_MILLISECONDS := "200"
export BURST_SIZE := "10"

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

# creates a new up and down migration
new-migration name:
    sqlx migrate add -r {{name}}

# runs the migrations
run-migrations:
    sqlx migrate run

# revert the last migration
revert-migration:
    cargo sqlx migrate revert