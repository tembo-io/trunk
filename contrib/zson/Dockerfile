# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/postgrespro/zson.git

# Set project version
ARG RELEASE=v1.1

# Build extension
RUN cd zson && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
