FROM rust:alpine3.16 as builder

LABEL maintainer="Yggdrasil80 <louisdechorivit@gmail.com>"

WORKDIR /usr/src/app
RUN apk upgrade --update-cache --available && apk add musl-dev zlib-dev openssl-dev && rm -rf /var/cache/apk/*

COPY . .
RUN cargo build --release

FROM alpine:3.16

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/doc-storage .

ENTRYPOINT ["./doc-storage"]