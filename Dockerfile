FROM --platform=$BUILDPLATFORM lukemathwalker/cargo-chef:latest-rust-alpine3.16 AS chef

WORKDIR /usr/src/app

FROM --platform=$BUILDPLATFORM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM --platform=$BUILDPLATFORM chef AS builder

ARG CPU_CORES

COPY --from=planner /usr/src/app/recipe.json recipe.json

RUN apk upgrade --update-cache --available && apk add musl-dev zlib-dev openssl-dev && rm -rf /var/cache/apk/*
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release -j $CPU_CORES --package doc-storage --bin doc-storage

FROM --platform=$TARGETPLATFORM alpine:3.16.2 AS runtime

LABEL maintainer="nullptr.rs <nullptr.rs@gmail.com>"
LABEL org.opencontainers.image.description="Doc Storage, your self-hosted file and document storage service"

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/doc-storage .

ENTRYPOINT ["./doc-storage"]