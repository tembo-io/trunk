# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    libkrb5-dev \
    libselinux1-dev \
    libzstd-dev \
    liblz4-dev \
    libxslt1-dev \
    libpam0g-dev \
    zlib1g-dev

# Clone repository
RUN git clone https://github.com/2ndQuadrant/pglogical.git

# Set project version
ARG RELEASE=REL2_4_3

# Build extension
RUN cd pglogical && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
