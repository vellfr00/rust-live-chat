FROM rust:1.82.0

ENV ENVIRONMENT=docker

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo install --path .

EXPOSE 3000
CMD rust-live-chat --run=server --host=127.0.0.1 --port=3000