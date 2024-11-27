# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    libhiredis-dev

# Clone repository
RUN git clone https://github.com/pg-redis-fdw/redis_fdw.git

ARG PG_VERSION=17
# Set project version
ARG RELEASE=REL_${PG_VERSION}_STABLE

# Build extension
RUN cd redis_fdw && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
