export GATEWAY_CONFIG := "gateway/dev_config.ron"
export DATASTORE_CONFIG := "datastore/dev_config.ron"
export CSS_FILE_NAME := "style.css"

alias app := watch-app
alias gateway := watch-gateway
alias datastore := watch-datastore

done_message := "✅ done"

debug_output := "target/assets/debug"
release_output := "target/assets/release"
recompile_delay_seconds := "2"

# list all recipes
default:
    just --list

# check the code for compile errors
check:
    @cargo clippy --package app --lib --bin app --all-features
    @cargo clippy --package common
    @cargo clippy --package datastore --lib --bin cli --bin datastore
    @cargo clippy --package gateway

# command for rust analyzer check
rust-analyzer-check:
    @cargo clippy --package app --lib --bin app --all-features --message-format=json-diagnostic-rendered-ansi
    @cargo clippy --package common --message-format=json-diagnostic-rendered-ansi
    @cargo clippy --package datastore --lib --bin cli --bin datastore --message-format=json-diagnostic-rendered-ansi
    @cargo clippy --package gateway --message-format=json-diagnostic-rendered-ansi

#######################################
# css related recipes
#######################################

# builds the css
build-css:
    @mkdir -p {{debug_output}} {{release_output}}
    @grass style/main.scss --style compressed | tee {{debug_output}}/$CSS_FILE_NAME {{release_output}}/$CSS_FILE_NAME > /dev/null

# watch for css changes and rebuild continuously
watch-css:
    @echo "Building css continuously..."
    @watchexec --watch style 'just build-css'

#######################################
# app related recipes
#######################################

# builds the app in debug mode
build-app:
    @echo "Building app..."
    @cargo build --package app --bin app --target wasm32-unknown-unknown --features=hydrate
    @cd app && wasm-bindgen \
        --target web \
        --weak-refs \
        --no-typescript \
        --out-dir ../{{debug_output}} \
        ../target/wasm32-unknown-unknown/debug/app.wasm
    @echo {{done_message}}

# builds the app in release mode
build-app-release:
    @echo "Building app in release mode..."
    @cargo build --package app --bin app --target wasm32-unknown-unknown --features=hydrate --release
    @cd app && wasm-bindgen \
        --target web \
        --weak-refs \
        --no-typescript \
        --out-dir ../{{release_output}} \
        ../target/wasm32-unknown-unknown/release/app.wasm
    @wasm-opt -Oz {{release_output}}/app.wasm

# watch app for changes and rebuild continuously
watch-app:
    @cargo watch --clear --delay {{recompile_delay_seconds}} --ignore src/gateway -- just build-app

#######################################
# datastore related recipes
#######################################

# run the datastore cli
datastore-cli *args:
    @cargo run --package datastore --bin cli -- {{args}}

# runs the datastore in debug mode
run-datastore:
    @echo "Building datastore..."
    @cargo run --package datastore --bin datastore

# builds the datastore in release mode
build-datastore-release:
    @echo "Building datastore in release mode..."
    @cargo build --package datastore --bin datastore --release
    @echo {{done_message}}

# watch datastore for changes and rebuild continuously
watch-datastore:
    @cargo watch --clear --delay {{recompile_delay_seconds}} --ignore src/bin/cli.rs -- just run-datastore

#######################################
# gateway related recipes
#######################################

# runs the gateway in debug mode
run-gateway:
    @echo "Building gateway..."
    @cargo run --package gateway

# builds the gateway in release mode
build-gateway-release:
    @echo "Building gateway in release mode..."
    @cargo build --package gateway --release
    @echo {{done_message}}

# watch gateway for changes and rebuild continuously
watch-gateway:
    @cargo watch --clear --delay {{recompile_delay_seconds}} --watch gateway -- cargo run --package gateway
