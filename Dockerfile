FROM rust:1.95.0 AS builder

WORKDIR /app

# RUN sudo apt install musl-tools

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
COPY migrations ./migrations
RUN touch src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

# RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api /usr/local/bin/app

ENV PORT=8080

ENV ENV="prod"

EXPOSE 8080

CMD ["app"]