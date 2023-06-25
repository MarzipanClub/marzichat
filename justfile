export BACKEND_CONFIG := "backend/dev_config.ron"
export CSS_FILE_NAME := "style.css"
export SQLX_OFFLINE := "true"
export DEBUG_OUTPUT := "target/assets/debug"
export RELEASE_OUTPUT := "target/assets/release"

alias app := watch-app
alias backend := watch-backend

done_message := "✅ done"
recompile_delay_seconds := "2"

# list all recipes
default:
    just --list

# check the code for compile errors
check:
    @cargo clippy --package app --lib --bin app --all-features
    @cargo clippy --package backend
    @cargo clippy --package common

# command for rust analyzer check
rust-analyzer-check:
    @cargo clippy --package app --lib --bin app --all-features --message-format=json-diagnostic-rendered-ansi
    @cargo clippy --package backend --message-format=json-diagnostic-rendered-ansi
    @cargo clippy --package common --message-format=json-diagnostic-rendered-ansi

#######################################
# css related recipes
#######################################

# builds the css
build-css:
    @mkdir -p $DEBUG_OUTPUT $RELEASE_OUTPUT
    @grass style/main.scss --style compressed | tee $DEBUG_OUTPUT/$CSS_FILE_NAME $RELEASE_OUTPUT/$CSS_FILE_NAME > /dev/null

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
    @mkdir -p $DEBUG_OUTPUT
    @cp -r assets/* $DEBUG_OUTPUT
    @cargo build --package app --bin app --target wasm32-unknown-unknown --features=hydrate
    @cd app && wasm-bindgen \
        --target web \
        --weak-refs \
        --no-typescript \
        --out-dir ../$DEBUG_OUTPUT \
        ../target/wasm32-unknown-unknown/debug/app.wasm
    @echo {{done_message}}

# builds the app in release mode
build-app-release:
    @echo "Building app in release mode..."
    @mkdir -p $RELEASE_OUTPUT
    @cp -r assets/* $RELEASE_OUTPUT
    @cargo build --package app --bin app --target wasm32-unknown-unknown --features=hydrate --release
    @cd app && wasm-bindgen \
        --target web \
        --weak-refs \
        --no-typescript \
        --out-dir ../$RELEASE_OUTPUT \
        ../target/wasm32-unknown-unknown/release/app.wasm
    @wasm-opt -Oz $RELEASE_OUTPUT/app.wasm

# watch app for changes and rebuild continuously
watch-app:
    @cargo watch --clear --delay {{recompile_delay_seconds}} --watch app --ignore app/src/bin -- just build-app

#######################################
# backend related recipes
#######################################

# runs the backend in debug mode
run-backend:
    @echo "Building backend..."
    @cargo run --package backend

# builds the backend in release mode
build-backend-release:
    @echo "Building backend in release mode..."
    @cargo build --package backend --release
    @echo {{done_message}}

# watch backend for changes and rebuild continuously
watch-backend:
    @cargo watch --clear --delay {{recompile_delay_seconds}} --watch backend --watch app -- cargo run --package backend

#######################################
# migration related recipes
#######################################

# runs the migrations
run-migrations:
    @echo "Running migrations..."
    @cd backend && sqlx migrate run --database-url $DATABASE_URL
