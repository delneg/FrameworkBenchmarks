FROM rust:1.59-slim-bullseye

ENV AXUM_TECHEMPOWER_MONGODB_URL=mongodb://tfb-database:27017

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /axum
COPY ./src ./src
COPY ./templates ./templates
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./run.sh ./run.sh
RUN chmod +x ./run.sh

ENV RUSTFLAGS "-C target-cpu=native"
RUN cargo build --release
RUN cp ./target/release/axum-mongo ./target/release/axum-techempower

EXPOSE 8000

CMD ["./run.sh"]
