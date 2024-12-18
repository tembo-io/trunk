# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y \
    libkrb5-dev \
    libselinux1-dev \
    libzstd-dev \
    liblz4-dev \
    libxslt1-dev \
    libpam0g-dev \
    zlib1g-dev

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "REL$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}")" https://github.com/2ndQuadrant/${EXTENSION_NAME}.git \
    && make -C ${EXTENSION_NAME}
