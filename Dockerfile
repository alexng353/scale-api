FROM rust:1-bookworm AS template

ARG DATABASE_PUBLIC_URL
ENV DATABASE_URL=$DATABASE_PUBLIC_URL

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

FROM template AS migrator

WORKDIR /app

RUN cargo install sqlx-cli

COPY ./migrations /app/migrations

RUN echo "DATABASE_URL=$DATABASE_PUBLIC_URL" > .env
RUN cargo sqlx migrate run

FROM template AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM template AS runner

WORKDIR /app

COPY --from=builder /app/target/release/scale-api /app/scale-api

CMD [ "/app/scale-api" ]
