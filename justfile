export SERVER_CONFIG := "config.ron"
export CSS_FILE_NAME := "style.css"

alias a := watch-app
alias s := watch-server
alias c := check

app_features := "hydrate"
server_features := "ssr"
done_message := "✅ done"

debug_output := "target/assets/debug"
release_output := "target/assets/release"

# list all recipes
default:
    just --list

# check the code for compile errors
check:
    @cargo check --all-targets --all-features

# builds the css
build-css:
    @mkdir --parents {{debug_output}} {{release_output}}
    @grass src/style.scss --style compressed | tee {{debug_output}}/$CSS_FILE_NAME {{release_output}}/$CSS_FILE_NAME > /dev/null

# builds the app in debug mode
build-app:
    @echo "Building app..."
    @cargo build --bin app --target wasm32-unknown-unknown --features={{app_features}}
    @wasm-bindgen \
        --target web \
        --weak-refs \
        --no-typescript \
        --out-dir {{debug_output}} \
        target/wasm32-unknown-unknown/debug/app.wasm
    @echo {{done_message}}

# builds the app in release mode
build-app-release:
    @echo "Building app in release mode..."
    @cargo build --bin app --target wasm32-unknown-unknown --features={{app_features}} --release
    @wasm-bindgen \
        --target web \
        --weak-refs \
        --no-typescript \
        --out-dir {{release_output}} \
        target/wasm32-unknown-unknown/release/app.wasm
    @wasm-opt -Oz {{release_output}}/app.wasm

# watch app for changes and rebuild continuously
watch-app:
    @cargo watch --clear --delay 1 --ignore src/server -- just build-app

# runs the server in debug mode
run-server:
    @echo "Building server..."
    @cargo build --bin server --features={{server_features}}
    @echo {{done_message}}
    @./target/debug/server

# builds the server in release mode
build-server-release:
    @echo "Building server in release mode..."
    @cargo build --bin server --features={{server_features}} --release

# watch server for changes and rebuild continuously
watch-server:
    @cargo watch --clear --delay 1 --ignore src/app -- just run-server
