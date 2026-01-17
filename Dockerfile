FROM rust:latest

RUN apt-get update && apt-get install -y \
    libasound2-dev \
    libudev-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY assets ./assets

RUN cargo build --release

RUN mkdir -p /output && cp target/release/engine /output/engine

CMD ["cargo", "test"]
