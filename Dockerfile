FROM rust:1.45.2-alpine3.11
RUN mkdir /work
WORKDIR /work
COPY src ./src
COPY Cargo.lock ./
COPY Cargo.toml ./
RUN cargo build --release
ENTRYPOINT target/release/reversi_random