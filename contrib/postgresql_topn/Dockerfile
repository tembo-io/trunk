ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/citusdata/postgresql-topn.git ${EXTENSION_NAME} \
    && make -C ${EXTENSION_NAME}
