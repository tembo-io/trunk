# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/bashtanov/argm.git

# Set project version
ARG RELEASE=1.1.1

# Build extension
RUN cd argm && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
