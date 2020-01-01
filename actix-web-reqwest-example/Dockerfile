FROM rust:slim AS builder

RUN apt-get update -qq \
 && apt-get install -y -qq --no-install-recommends \
      build-essential \
      libxml2 \
      libssl-dev \
      pkg-config \
      openssl \
      libpq-dev \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/* /tmp/* /var/log/apt/* /var/log/alternatives.log /var/log/dpkg.log /var/log/faillog /var/log/lastlog

ADD Cargo.toml /root/Cargo.toml
ADD Cargo.lock /root/Cargo.lock
COPY src /root/src

WORKDIR /root

RUN cargo build --release

FROM rust:slim

COPY --from=builder /root/target/release/actix-web-reqwest-example /root/actix-web-reqwest-example

WORKDIR /root

CMD ["/root/actix-web-reqwest-example"]
