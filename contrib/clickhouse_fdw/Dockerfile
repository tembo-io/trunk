# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    libcurl4-openssl-dev \
    uuid-dev

# Clone repository
RUN git clone https://github.com/ildus/clickhouse_fdw.git

# Set project version
ARG RELEASE=1.4.0

# Build extension
RUN cd clickhouse_fdw && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    mkdir build && \
    cd build && \
    cmake .. && \
    make
