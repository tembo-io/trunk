# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/postgrespro/pg_wait_sampling.git

# Set project version
ARG RELEASE=v1.1.4

# Build extension
RUN cd pg_wait_sampling && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make USE_PGXS=1
