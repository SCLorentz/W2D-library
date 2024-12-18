# syntax=docker/dockerfile:1
FROM golang:bookworm
WORKDIR /app

# Install system dependencies and clear cache
RUN apt-get update && apt-get install -y build-essential git curl npm

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install wasm-pack
RUN npm install wasm-pack -g

# Copy and configure the run script
COPY cmd/run.sh /
RUN chmod +x /run.sh

# Settings
# create bashrc
RUN touch ~/.bashrc && chmod u+w ~/.bashrc
# customize bashrc
COPY cmd/.bash /
RUN echo '. /.bash' >> ~/.bashrc

# Environment Settings
ENV FLASK_APP=chrome-dino-game
EXPOSE 8000

# Command to launch the application
CMD ["flask", "run", "--host", "0.0.0.0", "--port", "8000"]