FROM rust:1.79-bookworm AS build

WORKDIR /
RUN cargo new padel
COPY Cargo.toml Cargo.lock /padel/
WORKDIR /padel
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
  set -e
  touch /padel/src/main.rs
  cargo build --release
EOF

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates openssl && \
    rm -rf /var/lib/apt/lists/*
COPY --from=build /padel/target/release/padel /padel
RUN chmod +x /padel

CMD ["/padel"]
