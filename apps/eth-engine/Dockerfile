FROM rust:1 AS chef
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN apt-get update && apt-get install -y clang
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --profile prod --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --profile prod --target "$(rustc -vV | sed -n "s|host: ||p")" --bin reaper-eth-engine --target-dir /target &&\
    mv "/target/$(rustc -vV | sed -n "s|host: ||p")/prod/reaper-eth-engine" /app/target/prod/reaper-eth-engine

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/prod/reaper-eth-engine /usr/local/bin
ENTRYPOINT ["/usr/local/bin/app"]

