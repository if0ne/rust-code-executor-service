FROM ekidd/rust-musl-builder:latest AS Builder

RUN sudo apt-get update
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