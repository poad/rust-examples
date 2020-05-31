FROM rust:slim as builder

COPY . /root/app

WORKDIR /root/app
RUN cargo clean && cargo build


FROM debian:buster-slim


COPY --from=builder /root/app/target/debug/rust-cassandra-example /root/app/rust-cassandra-example

RUN ls /root/app/

WORKDIR /root/app/

EXPOSE 8080

CMD [ "./rust-cassandra-example" ]
