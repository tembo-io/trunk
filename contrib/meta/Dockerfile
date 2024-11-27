# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/aquameta/meta.git

# Set project version
ARG RELEASE=v0.3.0

# Build extension
RUN cd meta && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
