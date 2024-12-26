# the other version using the curl command to install rust was working, but just in case I'll use this version
FROM debian:bullseye-slim
COPY cmd/ /root/

RUN rm ~/.bashrc \
    && mv ~/.bash ~/.bashrc

ENV PATH="/usr/local/go/bin:$PATH" \
    FLASK_APP=chrome-dino-game \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    RUST_VERSION=1.83.0

RUN apt-get update && apt-get install -y build-essential git curl

RUN curl -LO https://go.dev/dl/go1.23.4.linux-arm64.tar.gz \
    && tar -C /usr/local -xzf go1.23.4.linux-arm64.tar.gz \
    && rm go1.23.4.linux-arm64.tar.gz

RUN chmod +x ~/rustup-init \
    && ~/rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host aarch64-unknown-linux-gnu \
    && rm -rf ~/rustup-init \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME

EXPOSE 8000
CMD ["flask", "run", "--host", "0.0.0.0", "--port", "8000"]