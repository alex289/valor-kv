FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev

WORKDIR /app

COPY . .
RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/valor_kv ./

EXPOSE 6380

CMD ["./valor_kv"]
