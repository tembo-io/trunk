ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "VERSION_$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}")" https://github.com/${EXTENSION_NAME}/${EXTENSION_NAME}.git \
    && make -C ${EXTENSION_NAME}
