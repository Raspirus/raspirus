FROM rust:1.77.1-bookworm

WORKDIR /usr/app/raspirus
COPY . .

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get clean && apt-get update && apt-get upgrade -y
RUN apt-get update && apt-get install -y build-essential \
    libsqlite3-0 \
    libsqlite3-dev \
    wget \
    npm \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    gcc-aarch64-linux-gnu \
    libssl-dev \
    libwebkit2gtk-4.0-dev

# Add architecture
RUN dpkg --add-architecture arm64 \
    && apt-get -qq update \
    && apt-get -qq install -y libwebkit2gtk-4.0-dev:arm64 libssl-dev:arm64

# Add Rust target
RUN rustup target add aarch64-unknown-linux-gnu

# Use latest npm version
RUN npm install -g npm@latest

# Perform npm install
RUN npm install

# Create the out directory
RUN mkdir out

# Install app deps
RUN cargo install --path src-tauri/
RUN cargo install tauri-cli --locked

# Set environment variables
ENV PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu/

# Build app
RUN cargo tauri build --target aarch64-unknown-linux-gnu -b deb