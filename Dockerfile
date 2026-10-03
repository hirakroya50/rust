FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked


FROM gcr.io/distroless/cc-debian13:nonroot AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/rust /app/server
COPY openapi.json /app/openapi.json

ENV PORT=3002

EXPOSE 3002

ENTRYPOINT ["/app/server"]