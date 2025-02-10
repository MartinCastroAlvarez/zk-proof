# Use the official Rust image as the base image
FROM rust:1.84.1 as builder

# Create a new directory for the application
WORKDIR /usr/src/api

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src/

# Install rustfmt and clippy
RUN rustup component add rustfmt

# Format the code
RUN cargo fmt -- --check

# Run tests
RUN cargo test -- --test-threads=1

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install OpenSSL and CA certificates
RUN apt-get update && \
    apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Expose the API port
EXPOSE 3030

# Set environment variables
ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug

# Run the binary
CMD ["./api"]
