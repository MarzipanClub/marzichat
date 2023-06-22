export BACKEND_CONFIG := "backend/sample_config.ron"
export CSS_FILE_NAME := "style.css"
export SQLX_OFFLINE := "true"

alias app := watch-app
alias backend := watch-backend

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
    @mkdir -p {{debug_output}}
    @cp -r assets/* {{debug_output}}
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
    @mkdir -p {{release_output}}
    @cp -r assets/* {{release_output}}
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
