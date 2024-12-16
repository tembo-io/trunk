ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y \
    build-essential \
    libcurl4-openssl-dev \
    liblz4-dev \
    libzstd-dev

# ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/hydradatabase/hydra.git \
    && cd hydra/columnar \
    && ./configure \
    && make
