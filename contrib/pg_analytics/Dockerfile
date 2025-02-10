ARG PG_VERSION
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.7

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

# Install Rust and download & build the extension.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && /root/.cargo/bin/rustup default stable \
    && curl -O https://api.pgxn.org/dist/${EXTENSION_NAME}/${EXTENSION_VERSION}/${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    && unzip ${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    && cd ${EXTENSION_NAME}-${EXTENSION_VERSION} \
    && cargo pgrx init --pg${PG_VERSION}=/usr/bin/pg_config \
    && cargo pgrx package
WORKDIR /app/${EXTENSION_NAME}-${EXTENSION_VERSION}
