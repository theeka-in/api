FROM node:24-alpine as migrator

WORKDIR /app

COPY package.json ./

RUN npm install

COPY prisma ./prisma

COPY scripts ./scripts

COPY migrations ./migrations

RUN node scripts/sqlx-migrations.js

RUN rm -rf migrations/_prisma

FROM rust:1.95.0 AS builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

COPY src ./src
COPY .sqlx ./.sqlx
COPY --from=migrator /app/migrations /app/migrations
RUN ls -la /app/migrations/

ENV SQLX_OFFLINE=true
ENV RUSTFLAGS="-A warnings"

RUN touch src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/theeka_api /usr/local/bin/app

ENV PORT=8080

ENV ENV="prod"

EXPOSE 8080

CMD ["app"]