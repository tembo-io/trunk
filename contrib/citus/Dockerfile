# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y \
    libcurl4-openssl-dev \
    liblz4-dev \
    libzstd-dev \
    libkrb5-dev \
    libselinux1-dev \
    libxslt1-dev

    # Download and build extension
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN curl -O https://api.pgxn.org/dist/${EXTENSION_NAME}/${EXTENSION_VERSION}/${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    && unzip ${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    && cd ${EXTENSION_NAME}-${EXTENSION_VERSION} \
    && ./configure && make
WORKDIR /app/${EXTENSION_NAME}-${EXTENSION_VERSION}
