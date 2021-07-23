FROM rust:1.53.0

WORKDIR /app

RUN apt-get update -qq && \
    cargo install diesel_cli && \
    cargo install cargo-watch

COPY . .
