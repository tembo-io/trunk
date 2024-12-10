# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build extension
RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/aws/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} && \
    && make
