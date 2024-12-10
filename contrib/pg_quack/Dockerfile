ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_VERSION
ARG EXTENSION_NAME

RUN git clone https://github.com/hydradatabase/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout v${EXTENSION_VERSION} \
    && make
