# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/petere/plsh.git

# Set project version
ARG RELEASE=1.20220917

# Build extension
RUN cd plsh && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
