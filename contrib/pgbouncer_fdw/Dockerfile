# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/CrunchyData/pgbouncer_fdw.git

# Set project version
ARG RELEASE=v1.1.0

# Build extension
RUN cd pgbouncer_fdw && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
