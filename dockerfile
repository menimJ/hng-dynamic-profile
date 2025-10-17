# --- Builder ---
FROM rust:1.82-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo 'fn main(){}' > src/main.rs && cargo build --release || true
COPY src ./src
RUN cargo build --release --locked

# --- Runtime ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    update-ca-certificates && useradd -m appuser && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# adjust name if your binary differs
COPY --from=builder /app/target/release/dynamic-profile-endpoint /usr/local/bin/app
RUN chmod +x /usr/local/bin/app

ENV RUST_LOG=info,axum=info,tower_http=info
ENV PORT=8080
EXPOSE 8080

USER appuser
CMD ["app"]
