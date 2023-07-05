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
