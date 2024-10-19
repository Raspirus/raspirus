FROM rust:1.82.0-bookworm

WORKDIR /usr/app/raspirus
COPY . .

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get clean && apt-get update && apt-get upgrade -y
RUN apt-get update && apt-get install -y pkg-config \
    build-essential \
    curl \
    libssl-dev

# Install Rust packager
RUN cargo install cargo-packager --locked

# Package app
RUN cargo packager --release --verbose
