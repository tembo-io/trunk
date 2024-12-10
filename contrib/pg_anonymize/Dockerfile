# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG RELEASE=692b354

# Build extension
RUN git clone https://github.com/rjuju/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout ${RELEASE} \
    && make
