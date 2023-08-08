# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/petere/pguint.git

# Set project version
ARG RELEASE=1.20220601

# Build extension
RUN cd pguint && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
