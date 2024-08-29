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

ARG PG_ANALYTICS_VERSION=v0.1.2
ARG PG_VERSION=15

# Build extension
RUN cd pg_analytics && \
	git fetch origin ${PG_ANALYTICS_VERSION} && \
	git checkout ${PG_ANALYTICS_VERSION} && \
    cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config && \
    cargo pgrx package
