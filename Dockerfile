FROM ghcr.io/rust-lang/rust:nightly-alpine AS chef
WORKDIR /app
RUN apk upgrade
RUN apk add musl-dev=1.2.2-r7
RUN cargo install --version 0.1.35 cargo-chef

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

RUN apk add rust=1.56.1-r0
RUN apk add openjdk17=17.0.3_p7-r0
RUN apk add python3=3.9.7-r4
RUN apk add nodejs=16.14.2-r0
RUN apk add zlib=1.2.12-r0
RUN apk add --no-cache mono=6.12.0.122-r1 --repository http://dl-cdn.alpinelinux.org/alpine/edge/testing
RUN apk add unzip=6.0-r9
RUN apk add wget=1.21.2-r2
RUN apk add bash=5.1.16-r0
#############################################################################
#It is not possible to specify the version for the pascal compiler, or the  #
#possibility was not found.                                                 #
#############################################################################
RUN wget http://pascalabc.net/downloads/PABCNETC.zip -O /tmp/PABCNETC.zip &&\
    mkdir /opt/pabcnetc &&\
    unzip /tmp/PABCNETC.zip -d /opt/pabcnetc

RUN wget https://github.com/JetBrains/kotlin/releases/download/v1.6.21/kotlin-compiler-1.6.21.zip -O /tmp/KOTLINC.zip &&\
    unzip /tmp/KOTLINC.zip -d /opt/

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-code-executor-service /usr/src/app/

EXPOSE 8000
CMD /usr/src/app/rust-code-executor-service