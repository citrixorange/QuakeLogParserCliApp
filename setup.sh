#!/bin/sh

# Install build tools and dependencies
apk update && apk add --no-cache \
    build-base \
    git \
    curl \
    gcc \
    musl-dev \
    openssl-dev \
    ca-certificates

# Install Rust and Cargo
curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal

