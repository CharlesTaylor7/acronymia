FROM lukemathwalker/cargo-chef:latest as chef
RUN rustup default nightly-2023-09-07
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --locked
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock .
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --features=ssr --bin=acronymia --target-dir=target/server 
RUN cargo chef cook --release --features=hydrate --target-dir=target/front --target=wasm32-unknown-unknown 
COPY . .
RUN cargo leptos build -vv --release

FROM debian:stable-slim AS runtime
WORKDIR /app
# Copy the server binary to the /app directory
COPY --from=builder /app/target/server/release/acronymia /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/
# Copy miscellaneous assets into the /app directory
# e.g. assets/prompts.txt
COPY --from=builder /app/assets /app/assets

ENTRYPOINT ["/usr/local/bin/app"]
