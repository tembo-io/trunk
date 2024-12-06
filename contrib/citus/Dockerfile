# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    libcurl4-openssl-dev \
    liblz4-dev \
    libzstd-dev \
    libkrb5-dev \
    libselinux1-dev \
    libxslt1-dev

# Clone repository
RUN git clone https://github.com/citusdata/citus.git

# Set project version
ARG RELEASE=v12.1.6

# Build extension
RUN cd citus && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    ./configure && \
    make
