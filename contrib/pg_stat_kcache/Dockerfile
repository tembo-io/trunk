ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN git clone --depth 1 --branch REL$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}") https://github.com/powa-team/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} && \
    && make
