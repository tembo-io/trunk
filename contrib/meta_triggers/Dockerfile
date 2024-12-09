# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/aquameta/meta_triggers.git

# Set project version
ARG RELEASE=v0.4.0

# Build extension
RUN cd meta_triggers && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
