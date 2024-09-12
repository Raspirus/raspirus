FROM rust:1.81.0-bookworm

WORKDIR /usr/app/raspirus
COPY . .

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get clean && apt-get update && apt-get upgrade -y
RUN apt-get update && apt-get install -y pkg-config \
                                           build-essential \
                                           curl \
                                           libssl-dev \
                                           gcc-aarch64-linux-gnu

# Add architecture
RUN dpkg --add-architecture arm64 \
    && apt-get -qq update \
    && apt-get -qq install -y libssl-dev:arm64

# Add Rust target
RUN rustup target add aarch64-unknown-linux-gnu

# Install cargo packager
RUN cargo install cargo-packager --locked

# Set environment variables
ENV PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu/

# Build app
RUN cargo build --release --target aarch64-unknown-linux-gnu

# Package app
RUN cargo packager --release --verbose
