FROM lukemathwalker/cargo-chef:latest-rust-1.58.1 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.58.1-alpine AS Builder

RUN apk upgrade
RUN apk add musl-dev

COPY --from=planner /app/recipe.json recipe.json
RUN cargo install cargo-chef --locked
RUN cargo chef cook --release --recipe-path recipe.json

COPY src src
COPY Cargo.lock .
COPY Cargo.toml .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM alpine AS Runner

WORKDIR /usr/src/app

COPY --from=builder /target/x86_64-unknown-linux-musl/release/rust-code-executor-service /bin

RUN apk add rust
RUN apk add openjdk17
RUN apk add python3
RUN apk add nodejsA

EXPOSE 8000
CMD rust-code-executor-service