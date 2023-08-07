# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/timescale/pg_prometheus.git

# Set project version
ARG RELEASE=0.2.2

# Build extension
RUN cd pg_prometheus && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
