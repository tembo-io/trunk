ARG PG_VERSION
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.5

USER root
RUN apt-get update && apt-get install -y \
	build-essential \
	libssl-dev \
	clang \
	cmake \
	libclang-dev \
    libopenblas-dev \
	pkg-config

ARG PG_VERSION=17
ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Upgrade Rust and build the extension.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
	&& $HOME/.cargo/bin/cargo pgrx init --pg$PG_VERSION $(which pg_config) \
	&& /root/.cargo/bin/rustup default stable \
	&& git clone --depth 1 --branch ${EXTENSION_VERSION} https://github.com/timescale/${EXTENSION_NAME}.git \
	&& cd ${EXTENSION_NAME}/${EXTENSION_NAME} \
	&& cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config \
	&& RUSTFLAGS="-C target-feature=+avx2,+fma" cargo pgrx package
