# Build application
FROM rust:1.77-slim-bookworm as builder

WORKDIR /app
COPY . .

RUN cargo build --release


# Copy application into Debian image
FROM gcr.io/distroless/cc-debian12

EXPOSE 9186

COPY --from=builder /app/target/release/brightness-exporter /
ENTRYPOINT [ "/brightness-exporter" ]