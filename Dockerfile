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

COPY src-tauri src-tauri
COPY out out

RUN cargo install --path src-tauri

FROM node:alpine

ENV USER=app
ENV APP_HOME=/home/$USER/app

RUN adduser -D $USER
WORKDIR $APP_HOME

COPY --from=build $APP_HOME/src-tauri/target/release/app .
COPY package*.json .
COPY public public
COPY components components
COPY pages pages
COPY out out
COPY services services
COPY styles styles
COPY next.config.js .
COPY postcss.config.js .
COPY tailwind.config.js .
COPY src-tauri/tauri.conf.json src-tauri/tauri.conf.json

RUN chown -R $USER:$USER $APP_HOME
USER $USER

RUN npm install

CMD ["npx", "tauri", "build"]