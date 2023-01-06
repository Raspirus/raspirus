FROM rust:latest as build

ENV USER=app
ENV APP_HOME=/home/$USER/app

RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    cmake \
    git \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libwebkit2gtk-4.0-dev \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home $USER
WORKDIR $APP_HOME

COPY --chown=$USER:$USER src-tauri src-tauri

RUN mkdir -p ../out

RUN cargo install --path src-tauri

FROM node:alpine

ENV USER=app
ENV APP_HOME=/home/$USER/app

RUN adduser -D $USER
WORKDIR $APP_HOME

COPY --from=build --chown=$USER:$USER $APP_HOME/target/release/tauri-bundler .
COPY --from=build --chown=$USER:$USER $APP_HOME/package*.json .
COPY --from=build --chown=$USER:$USER $APP_HOME/public public
COPY --from=build --chown=$USER:$USER $APP_HOME/src src
COPY --from=build --chown=$USER:$USER $APP_HOME/src-tauri src-tauri

USER $USER

RUN npm ci

CMD ["npm", "run", "tauri:build"]