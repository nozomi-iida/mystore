FROM rust:1.53.0

WORKDIR /app

COPY . .

RUN apt-get update -qq && \
    cargo install diesel_cli && \
    cargo install cargo-watch
