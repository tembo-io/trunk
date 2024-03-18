# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    libkrb5-dev

# Clone repository
RUN git clone https://github.com/timescale/timescaledb.git

# Set project version
ARG RELEASE=2.13.1

# Build extension
RUN cd timescaledb && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    ./bootstrap -DAPACHE_ONLY=1 && \
    cd build && \
    make
