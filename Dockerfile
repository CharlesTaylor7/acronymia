FROM lukemathwalker/cargo-chef:latest as chef
RUN rustup default nightly-2023-09-07
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock .
COPY ./src ./src
RUN cargo chef prepare --features=ssr --recipe-path=ssr-recipe.json

FROM chef AS builder
COPY --from=planner /app/ssr-recipe.json .
RUN cargo chef cook --release --features=ssr --recipe-path=ssr-recipe.json
COPY . .
RUN cargo build --release --features=ssr
RUN mv ./target/release/<your-crate> ./app

FROM debian:stable-slim AS runtime
WORKDIR /app
COPY --from=builder /app/app /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/app"]
