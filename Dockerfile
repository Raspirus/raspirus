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
                                           librsvg2-dev

# Create the out directory
RUN mkdir dist

# Add the rust toolchain
RUN rustup target add wasm32-unknown-unknown

# Install app deps
RUN cargo install tauri-cli
RUN cargo install trunk
RUN cargo install --path src-tauri/

# Build app
RUN cargo tauri build -b deb