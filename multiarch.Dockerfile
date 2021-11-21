FROM --platform=$BUILDPLATFORM rust:bullseye AS base-builder

ENV USER=bob

WORKDIR /code
RUN cargo init --lib
COPY Cargo.toml /code/Cargo.toml

RUN mkdir -p /code/.cargo \
  && cargo vendor > /code/.cargo/config

FROM rust:bullseye AS builder

RUN apt-get update \
  && apt-get install -y gcc g++

COPY --from=base-builder /code /code

COPY src /code/src
WORKDIR /code

RUN cargo build --release --offline

FROM debian:bullseye

COPY --from=builder /code/target/release/xiaomi-sensor-exporter /xiaomi-sensor-exporter

ENTRYPOINT ["/xiaomi-sensor-exporter"]
