FROM rust AS builder

RUN apt-get update -qq \
 && apt-get install -y -qq --no-install-recommends \
      build-essential \
      libxml2 \
 && apt-get clean

ADD Cargo.toml /root/Cargo.toml
ADD Cargo.lock /root/Cargo.lock
COPY src /root/src

WORKDIR /root

RUN cargo build --release

FROM rust:slim

COPY --from=builder /root/target/release/niconico-ranking /root/niconico-ranking

WORKDIR /root

CMD ["/root/niconico-ranking"]
