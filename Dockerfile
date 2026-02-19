FROM rust:1.77-slim AS builder

WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

COPY src ./src

RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/cowrie-api /usr/local/bin/cowrie-api

EXPOSE 3000

ENV COWRIE_LOG_PATH=/cowrie/var/log/cowrie/cowrie.json

CMD ["cowrie-api"]