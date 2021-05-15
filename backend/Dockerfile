FROM rust:latest
WORKDIR /usr/src/book-db
RUN rustup override set nightly
COPY . .
RUN cargo install --path .
ENTRYPOINT book-db