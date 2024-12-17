ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=v1.2-20240606
RUN git clone --depth 1 --branch "${RELEASE}" https://github.com/pgbigm/${EXTENSION_NAME}.git \
    && make -C ${EXTENSION_NAME} USE_PGXS=1
