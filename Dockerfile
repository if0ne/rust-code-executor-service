FROM lukemathwalker/cargo-chef:latest-rust-1.58.1 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM ekidd/rust-musl-builder:latest AS Builder

RUN sudo apt-get update

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

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-code-executor-service /usr/local/bin
EXPOSE 8000
CMD rust-code-executor-service