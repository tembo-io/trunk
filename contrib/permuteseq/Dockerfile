# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build extension
RUN git clone https://github.com/dverite/${EXTENSION_NAME}.git \
    && cd {EXTENSION_NAME} \
    && git checkout v${EXTENSION_VERSION} \
    && make
