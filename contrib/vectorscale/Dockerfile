ARG PG_VERSION=16
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.11.4
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
RUN $HOME/.cargo/bin/cargo install cargo-pgrx --version=0.11.4 --locked
ARG PG_VERSION=16
RUN $HOME/.cargo/bin/cargo pgrx init --pg$PG_VERSION $(which pg_config)

# # Set default Rust version
RUN /root/.cargo/bin/rustup default stable

# Clone repository
RUN git clone https://github.com/timescale/pgvectorscale.git

ARG EXTENSION_VERSION=0.2.0
ARG PG_VERSION=16

# Build extension
RUN cd pgvectorscale/pgvectorscale && \
	git fetch origin ${EXTENSION_VERSION} && \
	git checkout ${EXTENSION_VERSION} && \
    cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config && \
    RUSTFLAGS="-C target-feature=+avx2,+fma" cargo pgrx package