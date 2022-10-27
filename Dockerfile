FROM rust:1.64 AS builder

WORKDIR /work
RUN cargo new --bin szpp-judge-backend

WORKDIR /work/szpp-judge-backend
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release \
    && rm src/*.rs \
    && rm -f target/release/deps/szpp-judge-backend*
COPY ./src ./src

RUN cargo build --release

FROM ubuntu:22.04

COPY --from=builder /work/szpp-judge-backend/target/release/szpp-judge-backend .
ENTRYPOINT [ "/szpp-judge-backend" ]
