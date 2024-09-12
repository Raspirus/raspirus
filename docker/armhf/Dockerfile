FROM rust:1.81.0-bookworm

WORKDIR /usr/app/raspirus
COPY . .

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get clean && apt-get update && apt-get upgrade -y
RUN apt-get update && apt-get install -y pkg-config \
                                           build-essential \
                                           curl \
                                           libssl-dev \
                                           gcc-arm-linux-gnueabihf

# Add architecture
RUN dpkg --add-architecture armhf \
    && apt-get -qq update \
    && apt-get -qq install -y libssl-dev:armhf

# Add Rust target
RUN rustup target add armv7-unknown-linux-gnueabihf
RUN rustup target add wasm32-unknown-unknown

# Install cargo packager
RUN cargo install cargo-packager --locked

# Set environment variables
ENV PKG_CONFIG_SYSROOT_DIR=/usr/arm-linux-gnueabihf/

# Build app
RUN cargo build --release --target armv7-unknown-linux-gnueabihf

# Package app
RUN cargo packager --release --verbose
