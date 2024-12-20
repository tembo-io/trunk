# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y cmake libkrb5-dev

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "${EXTENSION_VERSION}" https://github.com/timescale/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && ./bootstrap -DAPACHE_ONLY=1 \
    && make -C build
