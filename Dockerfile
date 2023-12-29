FROM rust:1.75.0-bookworm

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
    libssl-dev \
    libwebkit2gtk-4.0-dev

# Perform npm install
RUN npm install

# Create the out directory
RUN mkdir out

# Install app deps
RUN cargo install --path src-tauri/
RUN cargo install tauri-cli

# Build app
RUN cargo tauri build -b deb