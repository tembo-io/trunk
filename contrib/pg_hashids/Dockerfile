ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN git clone https://github.com/iCyberon/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout v${EXTENSION_VERSION} \
    make
