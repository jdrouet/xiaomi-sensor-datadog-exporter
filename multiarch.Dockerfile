FROM --platform=$BUILDPLATFORM rust:latest AS base-builder

ENV USER=bob

WORKDIR /code
RUN cargo init --lib
COPY Cargo.toml /code/Cargo.toml

RUN mkdir -p /code/.cargo \
  && cargo vendor > /code/.cargo/config

FROM rust:latest AS builder

RUN apt-get update \
  && apt-get install -y gcc g++

COPY --from=base-builder /code /code

COPY src /code/src
WORKDIR /code

RUN cargo build --release --offline

FROM debian:buster

COPY --from=builder /code/target/release/xiaomi-sensor-datadog-export /xiaomi-sensor-datadog-export

CMD ["/xiaomi-sensor-datadog-export"]
