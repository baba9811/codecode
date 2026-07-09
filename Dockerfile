FROM node:24-bookworm-slim AS node
FROM rust:1-bookworm

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates python3 default-jdk-headless \
  && rm -rf /var/lib/apt/lists/*

COPY --from=node /usr/local/bin/node /usr/local/bin/node

WORKDIR /opt/practicode
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY assets ./assets
RUN cargo build --release --locked \
  && install -m 0755 target/release/practicode /usr/local/bin/practicode \
  && rm -rf target /usr/local/cargo/registry /usr/local/cargo/git

ENV HOME=/tmp
ENV PATH=/usr/local/cargo/bin:$PATH
ENV PRACTICODE_NO_UPDATE_CHECK=1
WORKDIR /workspace
ENTRYPOINT ["practicode"]
