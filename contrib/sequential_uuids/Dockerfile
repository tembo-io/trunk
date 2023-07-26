# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/tvondra/sequential-uuids.git

# Set project version
ARG RELEASE=v1.0.2

# Build extension
RUN cd sequential-uuids && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
