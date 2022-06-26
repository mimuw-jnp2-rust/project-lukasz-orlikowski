FROM rust

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked trunk

COPY . /app/
