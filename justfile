# used by sqlx-cli when creating/running/reverting migrations
export DATABASE_URL := "postgresql://marzichat@127.0.0.1/marzichat"

# used by marzichat to load the config
export CONFIG := "dev_config.ron"

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
watch args="":
    cargo leptos watch {{args}}

# creates a new up and down migration
new-migration name:
    sqlx migrate add -r {{name}}

# runs the migrations
run-migrations:
    sqlx migrate run

# revert the last migration
revert-migration:
    cargo sqlx migrate revert