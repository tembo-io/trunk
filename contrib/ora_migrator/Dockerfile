# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build extension
RUN git clone https://github.com/cybertec-postgresql/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} && \
    git fetch checkout RELEASE_$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}") && \
    make
