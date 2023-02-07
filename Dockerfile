FROM node:alpine as build

ENV USER=app
ENV APP_HOME=/home/$USER/app

RUN adduser -D $USER
WORKDIR $APP_HOME

COPY src-tauri src-tauri
COPY out out
COPY package*.json ./
COPY public public
COPY components components
COPY pages pages
COPY out out
COPY services services
COPY styles styles
COPY state state
COPY next.config.js .
COPY postcss.config.js .
COPY tailwind.config.js .
COPY src-tauri/tauri.conf.json src-tauri/tauri.conf.json

RUN chown -R $USER:$USER $APP_HOME
USER $USER

RUN npm install

RUN npm run build && npm run export

RUN npm install --save-dev @tauri-apps/cli

ENV PATH=$PATH:/usr/local/bin

# build the tauri app
RUN npm run tauri build

FROM node:alpine

ENV USER=app
ENV APP_HOME=/home/$USER/app

RUN adduser -D $USER
WORKDIR $APP_HOME

COPY --from=build $APP_HOME .

CMD ["npm", "start"]