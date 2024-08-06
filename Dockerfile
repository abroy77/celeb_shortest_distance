# Builder stage
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app/

RUN apt update && apt install lld clang -y

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release --bin celeb_app

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/* 

COPY --from=builder /app/target/release/celeb_app celeb_app

COPY configuration configuration

# move static files to the runtime image
COPY --from=builder /app/static static

# move actor db
COPY --from=builder /app/actors.db actors.db
# move movie_db csvs
COPY --from=builder /app/data/new_large data/new_large

ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./celeb_app"]