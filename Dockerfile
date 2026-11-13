FROM rust:slim-bookworm AS base
WORKDIR /app

FROM base AS dev
RUN cargo install cargo-watch
CMD ["cargo", "watch", "-x", "run"]

FROM base AS builder
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS prod
WORKDIR /app

COPY --from=builder /app/target/release/termtalk_server /usr/local/bin/termtalk_server

CMD ["termtalk_server"]