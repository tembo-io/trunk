ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG REVISION=1f36036

RUN git clone --depth 1 --branch "${REVISION}" https://github.com/vventirozos/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} && \
    && make
