# Build application
FROM rust:1.77-slim-bullseye as builder

WORKDIR /app
COPY . .

RUN cargo build --release


# Copy application into Debian image
FROM debian:bullseye-slim

EXPOSE 9186

COPY --from=builder /app/target/release/brightness-exporter /
ENTRYPOINT [ "/brightness-exporter" ]