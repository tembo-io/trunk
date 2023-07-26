# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/intgr/pg_financial.git

# Set project version
ARG RELEASE=v1.0.1

# Build extension
RUN cd pg_financial && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
