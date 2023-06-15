export SERVER_CONFIG := "config.ron"

alias a := watch-app
alias s := watch-server

app_features := "hydrate"
server_features := "ssr"
done_message := "✅ done"

# list all recipes
default:
    just --list

# builds the css
build-css:
    @mkdir --parents target/assets/debug target/assets/release
    @grass src/style.scss --style compressed | tee target/assets/debug/style.css target/assets/release/style.css > /dev/null

# builds the app in debug mode
build-app:
    @echo "Building app..."
    @cargo build --bin app --target wasm32-unknown-unknown --features={{app_features}}
    @wasm-bindgen \
        --target web \
        --weak-refs \
        --no-typescript \
        --out-dir target/assets/debug \
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
        --out-dir target/assets/release \
        target/wasm32-unknown-unknown/release/app.wasm
    @wasm-opt -Oz target/assets/release/app.wasm

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
