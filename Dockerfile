FROM rust:1.58.1-alpine AS chef
WORKDIR /app
RUN apk upgrade
RUN apk add musl-dev
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS Builder

RUN rustup target add x86_64-unknown-linux-musl

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM alpine AS Runner

WORKDIR /usr/src/app

RUN apk add rust
RUN apk add openjdk17
RUN apk add python3
RUN apk add nodejs

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-code-executor-service /usr/src/app/target/

EXPOSE 8000
CMD /usr/src/app/target/rust-code-executor-service