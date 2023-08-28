# Build Stage
FROM rust:latest as builder

# Set the target architecture for the Rust compiler
ENV RUSTFLAGS="-C target-cpu=native -C target-feature=+aes"

WORKDIR /usr/src/price-updater
COPY . .
RUN cargo build --release --bin price-updater --target aarch64-unknown-linux-gnu && \
    strip /usr/src/price-updater/target/aarch64-unknown-linux-gnu/release/price-updater

# Intermediate Stage for CA certificates
FROM debian:bookworm-slim as certs
RUN apt update && \
    apt install -y ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Runtime Stage
FROM arm64v8/debian:bookworm-slim

# Copy CA certificates
COPY --from=certs /usr/share/ca-certificates /usr/share/ca-certificates
COPY --from=certs /etc/ssl/certs /etc/ssl/certs

# Copy the binary
COPY --from=builder /usr/src/price-updater/target/aarch64-unknown-linux-gnu/release/price-updater /usr/local/bin/price-updater

# Install libssl-dev for SSL support
RUN apt update && \
    apt install -y libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

CMD ["price-updater"]
