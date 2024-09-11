# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/percona/pg_stat_monitor.git

# Set project version
ARG RELEASE=2.1.0

# Build extension
RUN cd pg_stat_monitor && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make USE_PGXS=1
