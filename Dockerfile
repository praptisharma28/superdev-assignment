# Dockerfile
FROM rust:latest as builder

WORKDIR /app
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/solana-http-server /usr/local/bin/solana-http-server

EXPOSE 3030

CMD ["solana-http-server"]