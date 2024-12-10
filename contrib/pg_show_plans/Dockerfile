# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y cmake

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build extension
RUN git clone https://github.com/cybertec-postgresql/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout v${EXTENSION_VERSION} \
    && make
