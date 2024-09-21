# syntax=docker/dockerfile:1
FROM golang:bookworm
WORKDIR /app

# install app dependencies (build-essential, git, curl, npm, rust)
RUN apt-get update && apt-get install -y build-essential git curl npm
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
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