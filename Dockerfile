FROM rust:1.76-buster as builder
WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./build.rs ./build.rs
COPY ./proto ./proto

RUN cargo build --release

FROM debian:buster-slim as runner

COPY --from=builder /usr/src/app/target/release/auth-server /usr/local/bin/auth-server
EXPOSE 50051

CMD ["auth-server"]
