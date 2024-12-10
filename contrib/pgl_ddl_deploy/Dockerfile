ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/enova/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} && \
    && make
