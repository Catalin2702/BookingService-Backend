FROM rust:1.99.0-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked \
    && cp target/release/booking-service /booking-service

FROM debian:bookworm-slim
COPY --from=builder /booking-service /usr/local/bin/booking-service
EXPOSE 3000
CMD ["booking-service"]
