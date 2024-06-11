ARG PG_VERSION=16
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.11.4
USER root

RUN apt-get update && apt-get install -y \
	build-essential \
	libssl-dev \
	clang \
	cmake \
	libclang-dev \
	pkg-config

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set default Rust version
RUN /root/.cargo/bin/rustup default stable

# Clone repository
RUN git clone https://github.com/timescale/pgvectorscale.git

ARG EXTENSION_VERSION=v0.2.0

# Build extension
RUN cd pgvectorscale && \
	git fetch origin v${EXTENSION_VERSION} && \
	git checkout v${EXTENSION_VERSION} && \
	make
