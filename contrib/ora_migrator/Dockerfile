# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "RELEASE_$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}")" https://github.com/cybertec-postgresql/${EXTENSION_NAME}.git \
    && make -C ${EXTENSION_NAME}
