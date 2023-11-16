FROM rust:1.74.0-bullseye
WORKDIR /usr/app

COPY ./ ./

# Add architecture
RUN dpkg --add-architecture arm64

# Add Tauri dependencies
RUN apt-get update && apt-get -y upgrade && apt-get install -y libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    npm \
    libwebkit2gtk-4.0-dev:arm64 \
    gcc-aarch64-linux-gnu \
    libssl-dev:arm64

# Add Rust target
RUN rustup target add aarch64-unknown-linux-gnu

# Perform npm install
RUN npm install

# Install app deps
RUN cargo install --path src-tauri/

# Set environment variables
RUN export PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu/

# Build app
RUN cargo tauri build --target aarch64-unknown-linux-gnu
