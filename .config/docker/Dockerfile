# renovate-automation: rustc version
FROM debian:bookworm-slim AS build

ARG SERVICE_DIR=eth-engine
ARG RELEASE=latest

# Set our working directory for the build

WORKDIR /usr/src/$SERVICE_DIR

# Ensure any Rust binaries that crash print a backtrace.
ENV RUST_BACKTRACE=1

# RUN sed -i -e 's#http://archive\.ubuntu\.com#http://us-east-1.ec2.archive.ubuntu.com#' \
#            -e 's#http://security\.ubuntu\.com#http://us-east-1.ec2.archive.ubuntu.com#' \
#            -e 's#http://ports\.ubuntu\.com#http://us-east-1.ec2.ports.ubuntu.com#' /etc/apt/sources.list

RUN apt-get update \
    && apt-get -qy install \
    gpg \
    sudo \
    wget \
    curl \
    dirmngr \
    gawk \
    && apt autoremove

RUN sudo install -dm 755 /etc/apt/keyrings

RUN wget -qO - https://mise.jdx.dev/gpg-key.pub | gpg --dearmor | sudo tee /etc/apt/keyrings/mise-archive-keyring.gpg 1> /dev/null
RUN echo "deb [signed-by=/etc/apt/keyrings/mise-archive-keyring.gpg arch=amd64] https://mise.jdx.dev/deb stable main" | sudo tee /etc/apt/sources.list.d/mise.list

RUN sudo apt update \
    && sudo apt install -y mise

# Copy the router source to our build environment
COPY . .

RUN cargo build -p reaper-eth-engine --profile prod

RUN mv /usr/local/cargo/bin/$SERVICE_DIR /dist

FROM amazonlinux:latest as distro

ARG DEBUG_IMAGE=false
ARG REPO_URL=https://github.com/storm-software/reaper
ARG PORT=9010

# Add a user to run the Ether Engine as
RUN useradd -m eth-engine

WORKDIR /dist

# Update apt and install ca-certificates
RUN \
  apt-get update -y \
  && apt-get install -y \
    ca-certificates

COPY --from=build --chown=root:root /dist /dist

ENV ENVIRONMENT production
ENV RELEASE $RELEASE
ENV PORT $PORT

# If debug image, install a bunch of useful debugging stuff and make a data directory
RUN \
  if [ "${DEBUG_IMAGE}" = "true" ]; then \
    apt-get install -y \
    dnsutils \
    procps \
    heaptrack \
    gdb \
    heaptrack-gui \
    x11-apps \
    iputils-ping && \
    mkdir data && \
    chown router data; \
  fi

# Clean up apt lists
RUN rm -rf /var/lib/apt/lists/*

HEALTHCHECK --interval=5s \
  --timeout=5s \
  --start-period=5s \
  --retries=6 \
  CMD $HEALTHCHECK_CMD

  # Create a wrapper script to run the router, use exec to ensure signals are handled correctly
RUN \
echo '#!/usr/bin/env bash \
\nset -e \
\n \
\nif [ -f "/usr/bin/heaptrack" ]; then \
\n    exec heaptrack -o /dist/data/eth-engine_heaptrack /dist/$SERVICE_DIR "$@" \
\nelse \
\n    exec /dist/$SERVICE_DIR "$@" \
\nfi \
' > /dist/entrypoint.sh

# Make sure we can run our wrapper
RUN chmod 755 /dist/entrypoint.sh

USER eth-engine

# Default executable is the wrapper script
ENTRYPOINT ["/dist/entrypoint.sh"]
