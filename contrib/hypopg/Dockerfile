ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "${EXTENSION_VERSION}" https://github.com/HypoPG/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && make
