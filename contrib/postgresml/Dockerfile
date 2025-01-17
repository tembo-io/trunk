ARG PG_VERSION=16
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.9

# Download and build the extension.
ARG PG_VERSION
ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y \
    bison flex \
	libopenblas-dev \
	python3 \
	python3-pip \
	libpython3-dev \
	lld

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && /root/.cargo/bin/rustup default stable \
    && git clone --depth 1 --branch v${EXTENSION_VERSION} https://github.com/${EXTENSION_NAME}/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME}/pgml-extension \
    && cargo pgrx init --pg${PG_VERSION}=/usr/bin/pg_config \
    && cargo pgrx package
