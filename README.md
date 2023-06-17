# marzichat

## Description
Marzichat is a lightweight and performant webapp for realtime chat. It's build entirely in Rust using idiomatic language features.

## Features

- TODO: write feature list

## Technologies Used

- [Axum](https://docs.rs/axum/latest/axum/) for the web server.
- [Leptos](https://docs.rs/leptos/latest/leptos/) for the frontend web framework
- [Redb](https://docs.rs/redb/latest/redb/) for the embedded key-value store

## Installation

1. Make sure you have Rust and PostgreSQL installed on your system.
2. Clone this repository.
3. Install [just](https://github.com/casey/just).
4. Install [cargo-watch](https://github.com/watchexec/cargo-watch).
5. Install [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen).
6. Install [grass](https://docs.rs/grass/latest/grass/).
7. Install [watchexec](https://github.com/watchexec/watchexec).
8. Install [wasm-opt](https://github.com/WebAssembly/binaryen) from source. Make sure to ad `wasm-opt` to your PATH:
```
    # .zshrc
    export PATH="$HOME/binaryen/bin:$PATH"
```
1. TODO: list installation steps

## Configuration

The application can be configured using a [ron](https://docs.rs/ron/0.8.0/ron/) config file.

TODO: explain configuration file

## Deployment

To deploy the application to a production server, follow these steps:

TODO: list steps to deploy

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [GNU GPLv3](https://choosealicense.com/licenses/gpl-3.0/) License.

## Acknowledgments

Special thanks to the Rust community and the developers of all the crates used in this project.