FROM rust:1.81.0-bookworm

WORKDIR /usr/app/raspirus
COPY . .

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get clean && apt-get update && apt-get upgrade -y
RUN apt-get update && apt-get install -y libwebkit2gtk-4.1-dev \
                                           build-essential \
                                           curl \
                                           wget \
                                           file \
                                           libxdo-dev \
                                           libssl-dev \
                                           libayatana-appindicator3-dev \
                                           librsvg2-dev \
                                           gcc-arm-linux-gnueabihf \

# Add architecture
RUN dpkg --add-architecture armhf \
    && apt-get -qq update \
    && apt-get -qq install -y libwebkit2gtk-4.1-dev:armhf libssl-dev:armhf

# Add Rust target
RUN rustup target add armv7-unknown-linux-gnueabihf
RUN rustup target add wasm32-unknown-unknown

# Create the out directory
RUN mkdir dist

# Install app deps
RUN cargo install trunk
RUN cargo install --path src-tauri/
RUN cargo install tauri-cli --locked

# Set environment variables
ENV PKG_CONFIG_SYSROOT_DIR=/usr/arm-linux-gnueabihf/

# Build app
RUN cargo tauri build --target armv7-unknown-linux-gnueabihf -b deb
