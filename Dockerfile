FROM rust:slim-bookworm AS base
WORKDIR /app

FROM base AS dev
RUN cargo install cargo-watch
CMD ["cargo", "watch", "-x", "run"]

FROM base AS builder
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src
COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim AS prod
WORKDIR /app
RUN groupadd -g 1000 appgroup && \
    useradd -u 1000 -g appgroup -s /bin/sh -m appuser
RUN mkdir -p /app/logs && chown -R appuser:appgroup /app
COPY --from=builder /app/target/release/termtalk_server /usr/local/bin/termtalk_server
USER appuser
CMD ["termtalk_server"]