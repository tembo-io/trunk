ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Install dependencies.
USER root
RUN apt-get update && \
    apt-get install -y \
    libreadline-dev \
    zlib1g-dev \
    flex bison \
    libxml2-dev \
    libxslt-dev \
    libxml2-utils \
    xsltproc \
    libc++-dev \
    libc++abi-dev \
    libglib2.0-dev \
    libtinfo6\
    libstdc++-12-dev \
    liblz4-dev \
    ninja-build

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/duckdb/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git submodule update --depth 1 --init --recursive \
    && make
