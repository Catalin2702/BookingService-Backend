FROM rust:1.99.0-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked \
    && cp target/release/booking_service /booking_service

FROM debian:bookworm-slim
COPY --from=builder /booking_service /usr/local/bin/booking_service
EXPOSE 3000
CMD ["booking_service"]
