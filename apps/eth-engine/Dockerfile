#
# === Build image ===
#
FROM rust:1.80-alpine3.20 AS chef

# Patching musl to use mimalloc, see docker/README.md.
# Source: https://github.com/tweag/rust-alpine-mimalloc
COPY docker/build.sh docker/mimalloc.diff /tmp/
RUN /tmp/build.sh
ENV LD_PRELOAD=/usr/lib/libmimalloc.so
ENV RUSTFLAGS="-C target-feature=+crt-static"

COPY rust-toolchain.toml rust-toolchain.toml
RUN apk add --no-cache musl-dev && cargo install --target "$(rustc -vV | sed -n "s|host: ||p")" cargo-chef
WORKDIR /apps/eth-engine

FROM chef AS planner
# At this stage we don't really bother selecting anything specific, it's fast enough.
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ENV CARGO_INCREMENTAL=0
COPY --from=planner /reaper/recipe.json recipe.json
RUN cargo chef cook --target "$(rustc -vV | sed -n "s|host: ||p")" --profile prod --recipe-path recipe.json

COPY /Cargo.lock /Cargo.lock
COPY /Cargo.toml /Cargo.toml
COPY /crates /crates
COPY /.config/engine /.config/engine

RUN cargo build --profile prod --target "$(rustc -vV | sed -n "s|host: ||p")" --bin reaper-eth-engine --target-dir ./target &&\
    mv "/target/$(rustc -vV | sed -n "s|host: ||p")/prod/reaper-eth-engine" /target/prod/reaper-eth-engine

#
# === Final image ===
#
FROM debian:bullseye-slim AS runtime

WORKDIR /apps/eth-engine

# used curl to run a health check query against the server in a docker-compose file
RUN apk add --no-cache curl

RUN adduser -g wheel -D reaper-eth-engine -h "/data" && mkdir -p /data && chown reaper-eth-engine: /data
USER reaper-eth-engine

COPY --from=builder /apps/eth-engine/target/prod/reaper-eth-engine /bin/reaper-eth-engine
COPY --from=builder /.config/engine /etc

VOLUME /data
WORKDIR /data

ENTRYPOINT ["/bin/reaper-eth-engine"]
CMD ["--config", "/etc", "--listen-address", "0.0.0.0:5000"]

EXPOSE 5000
