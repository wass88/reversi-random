FROM rust:1.48
RUN mkdir /work
WORKDIR /work
COPY src ./src
COPY Cargo.lock ./
COPY Cargo.toml ./
RUN cargo build --release

FROM alpine:latest
WORKDIR /work
COPY --from=0 /work/target/release/reversi_random .
ENTRYPOINT /work/reversi_random