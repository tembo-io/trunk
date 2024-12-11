# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build extension
RUN git clone --depth 1 --branch REL_$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}") https://github.com/cybertec-postgresql/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && make
