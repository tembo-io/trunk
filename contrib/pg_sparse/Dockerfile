# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root


# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/paradedb/paradedb.git \
    && make -C paradedb/${EXTENSION_NAME} USE_PGXS=1 OPTFLAGS=""
