# Description
The Rust programming language is really coming along with respect to fullstack development. This project has one goal - investigate the feasibility of using Rust for a fullstack microservices application.


# Requirements
- Dioxus CLI `0.7.0-rc.2`
- Rust
- Ubuntu 24.04.3

# How to use
- Run `make` to start all docker instances.

# Endpoints
`localhost:8080` - Surrealist DBMS.
`localhost:8000` - Surrealdb database.
`localhost:XXXX` - Web App (currently does not work).

# Stack
[Rust](https://rust-lang.org/) as the programming language of choice.
[Dioxus](https://dioxuslabs.com/) fullstack framework.
[SurrealDb](https://surrealdb.com/docs/surrealdb) as database.
[Surrealist](https://surrealdb.com/docs/surrealist) as the DMS.

# Issues
* Dioxus app still does not run with --web
* Dioxus app still not dockerized (need ngix or similar?).