# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN git clone --depth 1 --branch "${EXTENSION_VERSION}" https://github.com/percona/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} && \
    && make USE_PGXS=1
