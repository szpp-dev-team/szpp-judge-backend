FROM rust:1.16 AS builder

WORKDIR /work
RUN cargo new --bin dummy

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

COPY ./src .
RUN cargo build --release

FROM ubuntu:22.04

COPY --from=builder /work/dummy/target/release/dummy .
ENTRYPOINT [ "/dummy" ]
