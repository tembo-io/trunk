# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/darold/pgtt.git

# Set project version
ARG RELEASE=v2.10

# Build extension
RUN cd pgtt && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
