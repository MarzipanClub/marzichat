# marzichat

## Description
Marzichat is a lightweight and performant webapp for realtime chat. It's build entirely in Rust using idiomatic language features.

## Features

- TODO: write feature list

## Installation

### Requirements
1. Make sure you have [Rust](https://www.rust-lang.org) and [PostgreSQL](https://www.postgresql.org) installed on your system.
2. Clone this repository.
3. Install [just](https://github.com/casey/just): `cargo install just`
4. Install [cargo-watch](https://github.com/watchexec/cargo-watch): `cargo install cargo-watch`
5. Install [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen): `cargo install wasm-bindgen-cli`
<!-- 6. Install [grass](https://docs.rs/grass/latest/grass/): `cargo install grass` -->
7. Install [watchexec](https://github.com/watchexec/watchexec): `cargo install watchexec-cli`
8. Install [sqlx-cli](https://github.com/launchbadge/sqlx/tree/253d8c9f696a3a2c7aa837b04cc93605a1376694/sqlx-cli): `cargo install sqlx-cli --no-default-features --features postgres`
9.  Install [wasm-opt](https://github.com/WebAssembly/binaryen#building) from source. Make sure to add `wasm-opt` to your PATH:
```
    # .zshrc
    export PATH="$HOME/binaryen/bin:$PATH"
```
### Database setup
Create a database within postgres.
1. Type `sudo -u postgres psql` in you terminal to enter the postgres interpreter as the postgres user. This user has admin permissions to create more users.
2. Run `\du` to describe users.
3. Create a new user (called roles in postgres) with:

    `create role marzichat with login;`

4. Run `\du` to view the users again. You should see the new user.
5. Create the database:

    `create database marzichat with owner marzichat;`

6. Logout of psql:

    `\q`

7. Run the migrations:

    `DATABASE_URL="postgresql://marzichat@127.0.0.1/marzichat" just run-migrations`

## Configuration

The backend is configured using a [ron](https://docs.rs/ron/0.8.0/ron/) formatted file. (Ron is to Rust what JSON is to javascript.)
Create a new ron file based on `dev_config.ron` and update the `justfile` to:

    export BACKEND_CONFIG := "<path-to-your-config-ron-file>"

## Deployment

To deploy the application to a production server, follow these steps:

TODO: list steps to deploy

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [GNU GPLv3](https://choosealicense.com/licenses/gpl-3.0/) License.

## Acknowledgments

Special thanks to the Rust community and the developers of all the crates used in this project.