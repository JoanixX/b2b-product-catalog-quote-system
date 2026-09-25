FROM rust:1.96-bookworm AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static
COPY migrations ./migrations
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
RUN useradd --system --uid 10001 app
COPY --from=build /app/target/release/b2b_product_catalog_quote_system_api /usr/local/bin/trazo-api
USER app
ENV PORT=3000
EXPOSE 3000
ENTRYPOINT ["trazo-api"]
