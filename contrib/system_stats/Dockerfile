# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/EnterpriseDB/system_stats.git

# Set project version
ARG RELEASE=v1.0

# Build extension
RUN cd system_stats && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
