# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y \
    cmake \
    autoconf \
    automake \
    libtool \
    zlib1g-dev \
    libxml2-dev \
    libcunit1-dev

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/pg${EXTENSION_NAME}/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && ./autogen.sh \
    && ./configure \
    && make
