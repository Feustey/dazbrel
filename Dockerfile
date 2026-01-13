# Multi-stage build pour optimiser la taille
FROM rust:1.82 as builder

WORKDIR /app

ENV CARGO_BUILD_JOBS=1

RUN apt-get update && apt-get install -y \
    libssl-dev \
    libsqlite3-dev \
    protobuf-compiler \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release -j 1

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/dazno-umbrel ./
COPY templates/ ./templates/
COPY static/ ./static/

EXPOSE 3000

CMD ["./dazno-umbrel"]
