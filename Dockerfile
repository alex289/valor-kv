FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY src ./src
RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/valor_kv ./

EXPOSE 3000

CMD ["./valor_kv"]
