# This docker file is used to run the aggregator
FROM rust:slim-bookworm

ARG RUST_ROOT=src/rust
ARG DATABASE_URL
COPY $RUST_ROOT /app

RUN apt-get update && apt-get install -y \
    libudev-dev \
    build-essential \
    libclang-dev \
    libpq-dev \
    libssl-dev \
    lld \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN cargo build --release -p aggregator

ENV DATABASE_URL=$DATABASE_URL

ENTRYPOINT ["cargo", "run", "--release", "-p", "aggregator"]