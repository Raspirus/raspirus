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
COPY package*.json ./
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

# need to build the Next.js app before we build the tauri app
RUN npm run build && npm run export

RUN npm install --save-dev @tauri-apps/cli

ENV PATH=$PATH:/usr/local/bin

# build the tauri app
CMD ["sh", "-c", "npm run build && npm run export && npm run tauri build"]
