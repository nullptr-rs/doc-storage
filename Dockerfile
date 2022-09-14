ARG CPU_CORES

FROM --platform=$BUILDPLATFORM lukemathwalker/cargo-chef:latest-rust-alpine3.16 AS chef

LABEL maintainer="Yggdrasil80 <louisdechorivit@gmail.com>"
WORKDIR /usr/src/app

FROM --platform=$BUILDPLATFORM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM --platform=$BUILDPLATFORM chef AS cooker

COPY --from=planner /usr/src/app/recipe.json recipe.json

RUN apk upgrade --update-cache --available && apk add musl-dev zlib-dev openssl-dev && rm -rf /var/cache/apk/*
RUN cargo chef cook --release --recipe-path recipe.json

FROM --platform=$BUILDPLATFORM chef AS builder

COPY --from=cooker . .
RUN cargo build --release -j $CPU_CORES --package doc-storage --bin doc-storage

FROM --platform=$TARGETPLATFORM alpine:3.16.2 AS runtime

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/doc-storage .

ENTRYPOINT ["./doc-storage"]