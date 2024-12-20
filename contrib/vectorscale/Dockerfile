ARG PG_VERSION
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.5

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y \
	build-essential \
	libssl-dev \
	clang \
	cmake \
	libclang-dev \
    libopenblas-dev \
	pkg-config

ARG PG_VERSION
ARG EXTENSION_NAME
ARG EXTENSION_VERSION

USER root
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && /root/.cargo/bin/rustup default stable \
    && git clone --depth 1 --branch ${EXTENSION_VERSION} https://github.com/timescale/pg${EXTENSION_NAME}.git \
    && cd pg${EXTENSION_NAME}/pg${EXTENSION_NAME} \
    && cargo pgrx init --pg${PG_VERSION} $(which pg_config) \
    && RUSTFLAGS="-C target-feature=+avx2,+fma" cargo pgrx package
