FROM lukemathwalker/cargo-chef:latest-rust-1 AS init
WORKDIR /app

# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN apt-get update && apt-get install -y clang
RUN rustup toolchain install nightly-x86_64-unknown-linux-gnu
# RUN cargo install cargo-chef

FROM init AS prepare
COPY . .

RUN cargo +nightly chef prepare --recipe-path recipe.json

FROM init AS build
COPY --from=prepare /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN mkdir -p /app/target/prod
RUN cargo +nightly chef cook --profile prod --recipe-path recipe.json

COPY . .
RUN cargo +nightly build --profile prod --target "$(rustc -vV | sed -n "s|host: ||p")" --bin reaper-eth-engine --target-dir /target \
  && mv "/target/$(rustc -vV | sed -n "s|host: ||p")/prod/reaper-eth-engine" /app/target/prod/reaper-eth-engine

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=build /app/target/prod/reaper-eth-engine /usr/local/bin

RUN groupadd -g 999 reaper-eth-engine && \
    useradd -r -u 999 -g reaper-eth-engine reaper-eth-engine
USER reaper-eth-engine

ENTRYPOINT ["/usr/local/bin"]
CMD ["run", "-i", "AtomicArb,CexDex,Jit,Liquidations,Sandwich,SearcherActivity,JitCexDex", "-r", "8000", "--from-db-tip"]

EXPOSE 8000
