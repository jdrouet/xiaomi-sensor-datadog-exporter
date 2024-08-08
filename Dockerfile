FROM --platform=$BUILDPLATFORM rust:bookworm AS vendor

RUN apt-get update \
    && apt-get install -y libclang-dev libbluetooth-dev

ENV USER=bob

WORKDIR /code
RUN cargo init --lib
COPY Cargo.toml /code/Cargo.toml

RUN mkdir -p /code/.cargo \
    && cargo vendor > /code/.cargo/config

FROM rust:bookworm AS builder

RUN apt-get update \
    && apt-get install -y gcc g++ libclang-dev libbluetooth-dev libdbus-1-dev pkg-config

COPY --from=vendor /code /code

COPY src /code/src
WORKDIR /code

RUN cargo build --release --offline

FROM debian:bookworm

COPY --from=builder /code/target/release/xiaomi-sensor-exporter /xiaomi-sensor-exporter

ENTRYPOINT ["/xiaomi-sensor-exporter"]
