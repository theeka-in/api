FROM rust:1.95.0 AS builder

WORKDIR /app

RUN apt-get update
RUN apt-get install -y musl-tools

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

COPY src ./src
COPY .sqlx ./.sqlx
COPY migrations ./migrations
RUN ls -la /app/migrations/

ENV SQLX_OFFLINE=true
ENV RUSTFLAGS="-A warnings"

RUN touch src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/theeka_api /usr/local/bin/app

RUN apk add --no-cache ca-certificates

ENV ENV="prod"
ENV RUST_BACKTRACE=full

EXPOSE 404

CMD ["app"]