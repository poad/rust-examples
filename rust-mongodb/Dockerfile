FROM rust:slim AS build-env

COPY src /root/src
ADD Cargo.toml /root/Cargo.toml

WORKDIR /root

RUN cargo build --release

FROM rust:slim

COPY --from=build-env /root/target/release/rust_mongodb /root/rust_mongodb

RUN chmod 744 /root/rust_mongodb

ENV RUST_BACKTRACE 1

EXPOSE 3000

CMD ["/root/rust_mongodb"]
