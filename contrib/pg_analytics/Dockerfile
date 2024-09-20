ARG PG_VERSION=15
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.1
USER root

RUN apt-get update && apt-get install -y \
	build-essential \
	libssl-dev \
	clang \
	cmake \
	libclang-dev \
    libopenblas-dev \
	pkg-config

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Clone repository
RUN git clone https://github.com/paradedb/pg_analytics

# argument passed in during build and defined in Trunk.toml
ARG EXTENSION_VERSION

ARG PG_VERSION=15

# Build extension
RUN cd pg_analytics && \
	git fetch origin v${EXTENSION_VERSION} && \
	git checkout v${EXTENSION_VERSION} && \
    cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config && \
    cargo pgrx package
