FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked


FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       ca-certificates libgcc-s1 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 appuser

WORKDIR /app

COPY --from=builder /app/target/release/rust /app/server
COPY openapi.json /app/openapi.json

USER appuser

ENV PORT=3002

EXPOSE 3002

CMD ["/app/server"]