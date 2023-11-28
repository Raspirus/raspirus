ARG NODE_VERSION=18.18.2
ARG ALPINE_VERSION=3.17.5

FROM node:${NODE_VERSION}-alpine AS node
FROM arm64v8/alpine:${ALPINE_VERSION}

COPY --from=node /usr/lib /usr/lib
COPY --from=node /usr/local/lib /usr/local/lib
COPY --from=node /usr/local/include /usr/local/include
COPY --from=node /usr/local/bin /usr/local/bin

WORKDIR /usr/app/raspirus
COPY . .

RUN apk update && apk add --no-cache \
        openssl-dev musl-dev glib-dev cairo-dev pkgconfig gdk-pixbuf-dev webkit2gtk-dev curl \
        libappindicator-dev patchelf librsvg-dev gtk+3.0-dev nodejs npm make build-base

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add Rust to path
ENV PATH="/root/.cargo/bin:${PATH}"

# Add Rust target
RUN rustup target add aarch64-unknown-linux-musl

# Perform npm install
RUN npm install

# Add out directory
RUN mkdir out

# Install app deps
RUN cargo install --path src-tauri/

# Build app
RUN cargo tauri build --target aarch64-unknown-linux-musl