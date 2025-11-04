FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && apt-get install -y \
    libgtk-3-dev \
    build-essential \
    libjavascriptcoregtk-4.1-dev \
    libsoup-3.0-dev \
    libwebkit2gtk-4.1-dev

RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo install dioxus-cli --version 0.7.0
RUN dx bundle --web

FROM chef AS runtime
COPY --from=builder /app/target/dx/dx_rs/release/web/ /usr/local/app

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]