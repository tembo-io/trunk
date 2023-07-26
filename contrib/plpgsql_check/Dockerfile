# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/okbob/plpgsql_check.git

# Set project version
ARG RELEASE=v2.3.4

# Build extension
RUN cd plpgsql_check && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
