FROM rust:1.74-slim-buster
ENV TERM=xterm
WORKDIR /usr/src/raspirus
COPY . .
RUN apt-get update && apt-get install build-essential -y
RUN make build
