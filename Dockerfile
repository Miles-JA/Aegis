FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/aegis /app/aegis
COPY --from=builder /app/migrations app/migrations

EXPOSE 8001

CMD ["/app/aegis"]