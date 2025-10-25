FROM rust:1.90.0

ENV SRC_APP_DIR="/usr/src/app"

WORKDIR ${SRC_APP_DIR}

COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src ./src

RUN cargo build --release && mv ./target/release/file_processor /usr/local/bin/file_processor

EXPOSE 7001
ENTRYPOINT [ "/usr/local/bin/file_processor" ]
