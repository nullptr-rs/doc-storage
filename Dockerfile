FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef

LABEL maintainer="Yggdrasil80 <louisdechorivit@gmail.com>"
WORKDIR /usr/src/app

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /usr/src/app/recipe.json recipe.json

RUN apk upgrade --update-cache --available && apk add musl-dev zlib-dev openssl-dev && rm -rf /var/cache/apk/*
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin doc-storage

FROM alpine:3.16

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/doc-storage .

ENTRYPOINT ["./doc-storage"]