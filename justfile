export BACKEND_CONFIG := "backend/local_config.ron"
export RUSTFLAGS := "-Z macro-backtrace"
export DEBUG_OUTPUT := "target/assets/debug"
export RELEASE_OUTPUT := "target/assets/release"

alias app-assets := watch-app-assets
alias backend := watch-backend

done_message := "✅ done"
recompile_delay_seconds := "2"

# list all recipes
default:
    just --list

# check the code for compile errors
check:
    @cargo clippy --package app --lib --bin app --features=hyrate
    @cargo clippy --package backend
    @cargo clippy --package common

# command for rust analyzer check
rust-analyzer-check:
    @cargo clippy --package app --lib --bin app --features=hydrate --message-format=json-diagnostic-rendered-ansi
    @cargo clippy --package backend --message-format=json-diagnostic-rendered-ansi
    @cargo clippy --package common --message-format=json-diagnostic-rendered-ansi

#######################################
# asset related recipes
#######################################

# builds the app in debug mode
build-app-assets:
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
    @echo "✅ finished building app assets"

# builds the assets in release mode
build-app-assets-release:
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
watch-app-assets:
    @cargo watch --clear --delay {{recompile_delay_seconds}} --watch app --ignore app/src/bin -- just build-app-assets

#######################################
# backend related recipes
#######################################

# runs the backend in debug mode
run-backend: build-app-assets
    @echo "Building backend..."
    @cargo run --package backend

# builds the backend in release mode
build-backend-release:
    @echo "Building backend in release mode..."
    @cargo build --package backend --release
    @echo "✅ finished compiling backend"

# watch backend for changes and rebuild continuously
watch-backend:  build-app-assets
    @cargo watch --clear --delay {{recompile_delay_seconds}} --watch backend --watch common --watch app -- cargo run --package backend

#######################################
# migration related recipes
#######################################

# creates a new up and down migration
new-migration name:
    @echo "Creating migration..."
    @cd backend && sqlx migrate add -r {{name}}

# runs the migrations
run-migrations:
    @echo "Running migrations..."
    @cd backend && sqlx migrate run --database-url $DATABASE_URL

# revert the last migration
revert-migration:
    @echo "Reverting migration..."
    @cd backend && cargo sqlx migrate revert --database-url $DATABASE_URL
