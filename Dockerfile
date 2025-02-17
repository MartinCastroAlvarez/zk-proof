FROM ubuntu:22.04

# Install Ubuntu dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git

# Install Cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    cargo install rzup
ENV PATH="/root/.cargo/bin:${PATH}"

# Install rustfmt
RUN rustup component add rustfmt

# Install clippy
RUN rustup component add clippy

# Install risczero
# FIXME: Remove this
RUN git clone https://github.com/risc0/risc0

# Validate risczero installation
WORKDIR /risc0

# Install cargo-risczero
ENV CARGO_BUILD_JOBS=1
RUN cargo install cargo-risczero

# Validate risczero installation
RUN cargo risczero --version

# Install Docker in Docker
RUN apt-get update && apt-get install -y docker.io

# Install Docker Buildx
ENV DOCKER_BUILDKIT=1
RUN mkdir -p ~/.docker/cli-plugins && \
    curl -L https://github.com/docker/buildx/releases/download/v0.10.4/buildx-v0.10.4.linux-amd64 -o ~/.docker/cli-plugins/docker-buildx && \
    chmod +x ~/.docker/cli-plugins/docker-buildx

# Add the project to the image
ADD Cargo.toml .
ADD Cargo.lock .
ADD ./methods ./methods
ADD ./src ./src

# Run the project
ENTRYPOINT ["cargo", "run"]