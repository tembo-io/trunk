# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/pgq/pgq.git

# Set project version
ARG RELEASE=v3.5

# Build extension
RUN cd pgq && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
