FROM ghcr.io/rust-lang/rust:nightly-alpine AS chef
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
RUN cargo +nightly build --release --target x86_64-unknown-linux-musl


FROM alpine AS Runner

WORKDIR /usr/src/app

RUN apk add rust
RUN apk add openjdk17
RUN apk add python3
RUN apk add nodejs
RUN apk add zlib
RUN apk add --no-cache mono --repository http://dl-cdn.alpinelinux.org/alpine/edge/testing
RUN apk add unzip
RUN apk add wget
RUN apk add bash
RUN wget http://pascalabc.net/downloads/PABCNETC.zip -O /tmp/PABCNETC.zip &&\
    mkdir /opt/pabcnetc &&\
    unzip /tmp/PABCNETC.zip -d /opt/pabcnetc

RUN wget https://github.com/JetBrains/kotlin/releases/download/v1.6.21/kotlin-compiler-1.6.21.zip -O /tmp/KOTLINC.zip &&\
    unzip /tmp/KOTLINC.zip -d /opt/

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-code-executor-service /usr/src/app/

EXPOSE 8000
CMD /usr/src/app/rust-code-executor-service