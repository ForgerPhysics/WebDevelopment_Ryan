FROM rust:1.88-slim AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends libssl-dev ca-certificates pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend ./backend
COPY migrations ./migrations

ENV PORT=8000
EXPOSE 8000

CMD ["/app/backend"]
