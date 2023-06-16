export GATEWAY_CONFIG := "gateway/dev_config.ron"
export CSS_FILE_NAME := "style.css"

alias app := watch-app
alias gateway := watch-gateway

done_message := "✅ done"

debug_output := "target/assets/debug"
release_output := "target/assets/release"

# list all recipes
default:
    just --list

# check the code for compile errors
check:
    @cargo clippy --all-targets --all-features

# builds the css
build-css:
    @mkdir --parents {{debug_output}} {{release_output}}
    @grass style/main.scss --style compressed | tee {{debug_output}}/$CSS_FILE_NAME {{release_output}}/$CSS_FILE_NAME > /dev/null

# builds the app in debug mode
build-app:
    @echo "Building app..."
    @cd app && cargo build --bin app --target wasm32-unknown-unknown --features=hydrate
    @cd app && wasm-bindgen \
        --target web \
        --weak-refs \
        --no-typescript \
        --out-dir {{debug_output}} \
        ../target/wasm32-unknown-unknown/debug/app.wasm
    @echo {{done_message}}

# builds the app in release mode
build-app-release:
    @echo "Building app in release mode..."
    @cargo build --bin app --target wasm32-unknown-unknown --features=hydrate --release
    @wasm-bindgen \
        --target web \
        --weak-refs \
        --no-typescript \
        --out-dir {{release_output}} \
        ../target/wasm32-unknown-unknown/release/app.wasm
    @wasm-opt -Oz {{release_output}}/app.wasm

# watch app for changes and rebuild continuously
watch-app:
    @cargo watch --clear --delay 1 --ignore src/gateway -- just build-app

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
    @cargo watch --clear --delay 1 --ignore src/app -- cargo run --package gateway
