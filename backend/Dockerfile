FROM rust:latest as builder

RUN USER=root cargo new --bin book-db
WORKDIR ./book-db
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/book_db*
RUN cargo build --release


FROM debian:buster-slim

RUN apt-get update -y && \
    apt-get install -y libpq-dev

COPY --from=builder /book-db/target/release/book-db book-db
CMD ["/bin/bash", "-c", "ROCKET_WORKERS=1 ROCKET_PORT=$PORT ROCKET_DATABASES={book_db={url=$DATABASE_URL}} ./book-db"]
