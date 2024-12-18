ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "$(perl -E 'print shift =~ s/[.]0$//gr' "${EXTENSION_VERSION}")" https://github.com/ossc-db/${EXTENSION_NAME}.git \
    && make -C ${EXTENSION_NAME} USE_PGXS=1
