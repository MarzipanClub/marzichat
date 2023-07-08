# marzichat

## Description
Marzichat is a lightweight and performant web app for realtime chat. It's build entirely in Rust using idiomatic language features. The idea is to mimic old Reddit with its focus on long-form discussion forum.

## Features

- Actix http web server
- Leptos reactive frontend with support for server-side rendering
- Postgres
- Keydb
- Primer css

## Dev Setup
### Tooling setup
1. Clone this repository.
2. Install [Rust](https://www.rust-lang.org).
3. Install [just](https://github.com/casey/just): `cargo install just`
5. Install [Redis](https://redis.io/docs/getting-started/installation/) on your system.
6. Install [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli): `cargo install sqlx-cli --no-default-features --features native-tls,postgres`

### Postgres setup
1. Install [Postgres](https://www.postgresql.org/download/) on your system.
2. Create a database within postgres. Type `sudo -u postgres psql` in your terminal to enter the postgres interpreter as the postgres user. This user has admin permissions to create more users.
3. Run `\du` to describe users.
4. Create a new user (called roles in postgres) with:

    `create role marzichat with login;`

5. Run `\du` to view the users again. You should see the new user.
6. Create the database:

    `create database marzichat with owner marzichat;`

7. Logout of psql:

    `\q`

8. Run the migrations:

    `just run-migrations`

## Configuration

The backend is configured using environmental variables. The `justfile` contains a list of environmental variables that get exported when a recipe is invoked. The justfile can be changed to as required for development.

## Deployment

To deploy the application, you'll need to building in release mode with `cargo leptos build --release` and run the binary located in `target/release/marzichat`. The binary will need to be run with the environmental variables set.

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [GNU GPLv3](https://choosealicense.com/licenses/gpl-3.0/) License.

## Acknowledgments

Special thanks to the Rust community and the developers of all the crates used in this project.