# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y libhiredis-dev

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
ARG PG_VERSION
RUN git clone --depth 1 --branch "REL_${PG_VERSION}_STABLE" https://github.com/pg-redis-fdw/${EXTENSION_NAME}.git \
    && make -C ${EXTENSION_NAME}
