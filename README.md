# Description
The Rust programming language is really coming along with respect to fullstack development. This project has one goal - create a Rust fullstack microservices application (local and on gcp).

# Features
- ✅ Todo list (only for testing purposes).
- 🚧 Fastq parser
    - ✅ File upload.
    - ✅ Api endpoint.
    - ✅ MinIO storage for files.
    - ✅ NATS messaging to service.
    - ✅ NATS messaging receival in service.
    - ✅ Fastq processing.
    - 🚧 Database write.
    - 🚧 Frontend component.
- 🚧 Login with Google Account
    - ✅ Api endpoints.
    - ✅ Oauth functionality.
    - 🚧 Frontend login.

# Requirements
- Rust `>= 1.90`.
- Dioxus CLI `0.7.0-rc.3`.
- Ubuntu `24.04.3`.
- Docker `28.4.0`
- Docker Compose `2.39.2`

# Installation
## Rust Programming Language
Install via [Rustup](https://rustup.rs/). Optionally, run `rustup update` to get the latest Rust version.

## Dioxus CLI
Run `cargo install dioxus-cli --version 0.7.0-rc.3`. Alternatively, run `cargo binstall dioxus-cli@0.7.0-rc.3 --force` (requires `binstall`).

## Docker Engine
Getting Docker Engine is a bit tricky to get working, but following the official [Docker manual](https://docs.docker.com/engine/install/) should work.

# How to use
1. Run `make` to start docker instances.
2. `cd app`.
3. `dx serve --desktop`.

NOTE - due to issues related to dioxus rc-* versions, running `--web` does not currently work.

## Project structure

`api/` - Rust Axum API.<br>
`app/` - Dioxus frontend App.<br>
`compose/` - Docker compose files.<br>
`data/` - Volume mounted storage for minio, nats and surrealdb.<br>
`services/` - Rust microservices (workspace oriented).


## Environment file
In order for the application to work properly, a .env file is required on the `dx_rs` root directory. Currently, a single .env file is used but this is subject to change into service specific .env files later on.

```toml
# SurrealDB.
SURREALDB_ENDPOINT="db:8000"
ROOT_USERNAME="your_root_username"
ROOT_PASSWORD="your_root_password"
SURREALDB_NAMESPACE="your_namespace"
SURREALDB_DBNAME="your_db_name"

# MinIO.
MINIO_HTTP_ENDPOINT="http://minio:9000"
MINIO_ENDPOINT="minio:9000"
MINIO_ROOT_USER="your_minio_username"
MINIO_ROOT_PASSWORD="your_minio_password"

# NATS.
NATS_URL="nats://nats:4222"

# Google authentication.
GOOGLE_CLIENT_ID="your_google_client_id"
GOOGLE_CLIENT_SECRET="your_google_client_secret"
GOOGLE_REDIRECT_URL="where_to_redirect_upon_google_oauth_authentication"

# JWT
JWT_SECRET="your_random_jtw_secret"
```

# Endpoints
### App
`localhost:XXXX` - Web App (currently does not work).

### Database
`localhost:8080` - Surrealist DBMS.<br>
`localhost:8000` - Surrealdb database.<br>

### API
`localhost:8001` - Rust Axum API.<br>

### MinIO
`localhost:9001` - MinIO console.<br>

### NATS
`localhost:31311` - NATS console<br>

### Services
`localhost:7001` - Fastq service.<br>

# Stack
[Rust](https://rust-lang.org/) programming language.<br>
[Axum](https://github.com/tokio-rs/axum) as backend API.<br>
[Nats](https://github.com/nats-io) as message broker.<br>
[MinIO](https://github.com/minio/minio) for storage.<br>
[Dioxus](https://dioxuslabs.com/) as frontend.<br>
[SurrealDb](https://surrealdb.com/docs/surrealdb) as database.<br>
[Surrealist](https://surrealdb.com/docs/surrealist) as the DBMS.

# Overview
![diagram](https://github.com/OscarAspelin95/dx_rs/blob/8710a0ccfe26718bce9ed18e506542aabf2c135f/assets/diagram.svg)

# Issues
* Dioxus app still does not run with --web.
* Dioxus app still not dockerized (need ngix or similar?).

# TODO

## Main goals
* Add opentelemetry with prometheus and grafana for metrics and tracing.
* Explore SurrealDB capabilities.
* Await a stable Dioxus release.
* Enable login with google/github and optionally signup with email/password.
    * Fix oauth API endpoints.
    * Enable JWT with refresh tokens.
