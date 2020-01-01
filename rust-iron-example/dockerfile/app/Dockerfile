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

COPY app/Cargo.toml /root/Cargo.toml
COPY app/src /root/src

WORKDIR /root

RUN cargo build --release

FROM rust:slim

RUN apt-get update -qq \
 && apt-get install -y -qq --no-install-recommends \
      libpq5 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/* /tmp/* /var/log/apt/* /var/log/alternatives.log /var/log/dpkg.log /var/log/faillog /var/log/lastlog

COPY --from=builder /root/target/release/rust-iron-example /root/rust-iron-example
COPY ./dockerfile/app/assets/.env /root/.env

WORKDIR /root

EXPOSE 3000

CMD ["/root/rust-iron-example"]
