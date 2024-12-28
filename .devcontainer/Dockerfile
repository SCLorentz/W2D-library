FROM golang:bookworm

COPY cmd/ /root/

RUN chmod +x ~/run.sh \
    && rm ~/.bashrc \
    && mv ~/.bash ~/.bashrc

RUN apt-get update && apt-get install -y build-essential git curl npm

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && npm install wasm-pack -g

# Environment Settings
ENV FLASK_APP=chrome-dino-game
EXPOSE 8000

# Command to launch the application
CMD ["flask", "run", "--host", "0.0.0.0", "--port", "8000"]
