ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y libkrb5-dev

# Clone and build the extension.
ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG PG_VERSION
RUN git clone --depth 1 --branch "${PG_VERSION}.0" https://github.com/${EXTENSION_NAME}/${EXTENSION_NAME}.git \
    && make -C ${EXTENSION_NAME} USE_PGXS=1
