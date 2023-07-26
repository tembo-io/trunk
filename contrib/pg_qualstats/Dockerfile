# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/powa-team/pg_qualstats.git

# Set project version
ARG RELEASE=2.0.4

# Build extension
RUN cd pg_qualstats && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
