FROM rust:1.66.1-slim as builder
WORKDIR builder

COPY ./src ./src
COPY ./Cargo.toml .

RUN cargo build --release

FROM ubuntu:latest
EXPOSE "6379"
WORKDIR redis

COPY --from=builder /builder/target/release .

ENTRYPOINT ["/redis/redis"]