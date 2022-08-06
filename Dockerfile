FROM rust:bullseye AS builder

RUN apt-get update \
  && apt-get install -y libclang-11-dev libbluetooth-dev

ENV USER=bob

WORKDIR /code
RUN cargo init --lib
COPY Cargo.toml /code/Cargo.toml

RUN mkdir -p /code/.cargo \
  && cargo vendor > /code/.cargo/config

COPY src /code/src

RUN cargo build --release --offline

FROM scratch AS binary

COPY --from=builder /code/target/release/xiaomi-sensor-exporter /xiaomi-sensor-exporter

FROM debian:bullseye

COPY --from=builder /code/target/release/xiaomi-sensor-exporter /xiaomi-sensor-exporter

ENTRYPOINT ["/xiaomi-sensor-exporter"]
