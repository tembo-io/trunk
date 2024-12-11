# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build extension
RUN git clone https://github.com/powa-team/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout ${EXTENSION_VERSION} \
    && make
