FROM node:alpine as build

ENV USER=app
ENV APP_HOME=/home/$USER/app

RUN adduser -D $USER
WORKDIR $APP_HOME

COPY app/src-tauri src-tauri
COPY app/out out
COPY app/package*.json ./
COPY app/public public
COPY app/components components
COPY app/pages pages
COPY app/out out
COPY app/services services
COPY app/styles styles
COPY app/state state
COPY app/next.config.js .
COPY app/postcss.config.js .
COPY app/tailwind.config.js .
COPY app/src-tauri/tauri.conf.json src-tauri/tauri.conf.json

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