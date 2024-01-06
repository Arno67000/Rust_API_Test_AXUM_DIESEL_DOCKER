# RUST API - POSTGRES - AXUM - DIESEL -- (Dockerized)

## Prerequisites

This api has been built with rust 1.73.0, so you obiously need rust to be installed on your computer.

To install Rust or find informations, just follow the [Rust Documentation](https://www.rust-lang.org/fr/tools/install)

This api uses a postgres database. You'll need to set up the database locally or using docker to start this api.
For the api to work properly, the database must be initialized by migrations.
To run the migrations before starting the app :

- install diesel-cli : `cargo install diesel_cli --no-default-features --features postgres`
- make sure to transform the `.env.example` file to `.env` & `.env.docker.*.example` file to `.env.docker.*` & fill up the required values
- make sure the database is initialized locally or use docker: `docker compose up db -d`
- run `diesel migration run`

For more infos on Diesel ORM check out the [Diesel documentation](https://diesel.rs/)

## Migrations

_Adding migrations_:

```bash
diesel migration generate <migration-name>
```

_Running migrations_:

```bash
diesel migration run
```

_Testing migrations_:

```bash
diesel migration redo
```

## APP

_DATABASE MUST BE RUNNING for the api to work properly. You can set it up locally or use the dockerized one running `docker compose up db -d`_

```bash
# START
cargo run

# BUILD
cargo build

# BUILD FOR PRODUCTION
cargo build --release

# CHECK
cargo check

# CHECK WITH CLIPPY
cargo clippy

# BUILD DOCUMENTATION
cargo doc

# FORMAT CODE
cargo fmt
```

### EXECUTING IN DOCKER

_If you added some migrations, they must run before deploying :_

- run the database : `docker compose up db -d`
- run migrations: `diesel migration run`

**Run the containers :**

```bash
# Update docker image if you changed something in the code
docker compose build

# run both the database & the api
docker compose up -d
```

## API live tests :

I added a `.http` directory containing examples of the available calls to test the api once its running.
You can call the API from the .http files if you're on VsCode & if you have the Rest Client extension installed.
