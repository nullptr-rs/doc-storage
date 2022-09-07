FROM rust:latest as builder

LABEL maintainer="Yggdrasil80 <louisdechorivit@gmail.com>"

WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y libz-dev && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/doc-storage .

ENTRYPOINT ["./doc-storage"]