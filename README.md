# Description
The Rust programming language is really coming along with respect to fullstack development. This project has one goal - investigate the feasibility of using Rust for a fullstack microservices application.

# Requirements
- Rust `>= 1.90`.
- Dioxus CLI `0.7.0-rc.3`.
- Ubuntu `24.04.3`.
- Docker Engine.

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

# Endpoints
`localhost:8080` - Surrealist DBMS.<br>
`localhost:8000` - Surrealdb database.<br>
`localhost:8001` - Rust Axum API.<br>
`localhost:9001` - MinIO console.<br>
`localhost:XXXX` - Web App (currently does not work).

# Stack
[Rust](https://rust-lang.org/) programming language.<br>
[Axum](https://github.com/tokio-rs/axum) as backend API.<br>
[Nats](https://github.com/nats-io) as message broker.<br>
[MinIO](https://github.com/minio/minio) for storage.<br>
[Dioxus](https://dioxuslabs.com/) as frontend.<br>
[SurrealDb](https://surrealdb.com/docs/surrealdb) as database.<br>
[Surrealist](https://surrealdb.com/docs/surrealist) as the DBMS.

# Issues
* Dioxus app still does not run with --web.
* Dioxus app still not dockerized (need ngix or similar?).

# TODO

## Main goals
* Consider adding a Rust workspace for relevant Rust services.
* Add opentelemetry with prometheus and grafana for metrics and tracing.
* Explore SurrealDB capabilities.
* Await a stable Dioxus release.
* Implement NATS message queue.
* Enable login with google/github and optionally signup with email/password.
    * Fix oauth API endpoints.
    * Enable JWT with refresh tokens.