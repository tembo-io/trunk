ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION
ARG REVISION=537413f

# Clone repository
RUN git clone https://github.com/eulerto/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout ${REVISION} \
    && make
