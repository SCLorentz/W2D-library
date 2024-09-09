# syntax=docker/dockerfile:1
FROM golang:bookworm
WORKDIR /app

# install app dependencies
RUN apt-get update && apt-get install -y build-essential
RUN apt-get install -y git
RUN apt-get install -y curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# npm
RUN apt-get install -y npm
# Instalar wasm-pack
RUN npm install wasm-pack -g

# install app
COPY cmd/run.sh /
RUN chmod +x /run.sh

# Settings
# create bashrc
RUN touch ~/.bashrc && chmod u+w ~/.bashrc
# customize bashrc
COPY cmd/prompt /
RUN sh -c 'cat /prompt >> ~/.bashrc'
# run.sh

# final configuration
ENV FLASK_APP=chrome-dino-game
EXPOSE 8000
CMD flask run --host 0.0.0.0 --port 8000