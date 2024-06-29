ARG PG_VERSION=15
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.11.3
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

# Set default Rust version
RUN /root/.cargo/bin/rustup default stable

# Clone repository
RUN git clone https://github.com/paradedb/paradedb

ARG LAKEHOUSE_VERSION=v0.7.5
ARG PG_VERSION=15

# Build extension
RUN cd paradedb/pg_lakehouse && \
	git fetch origin ${LAKEHOUSE_VERSION} && \
	git checkout ${LAKEHOUSE_VERSION} && \
    cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config && \
    cargo pgrx package