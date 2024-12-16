# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/aquameta/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    # https://github.com/aquameta/meta/pull/39
    && printf "requires = 'meta'\n" >> ${EXTENSION_NAME}.control \
    && make
