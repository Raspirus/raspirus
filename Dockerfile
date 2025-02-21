FROM rust:1.85.0-bookworm

WORKDIR /usr/app/raspirus
COPY . .

ENV DEBIAN_FRONTEND=noninteractive

# Update and install dependencies
RUN apt-get clean && apt-get update && apt-get upgrade -y \
    && apt-get install -y pkg-config \
    build-essential \
    curl \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build the release binary
RUN cargo build --release 
