# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/pjungwir/aggs_for_vecs.git

# Set project version
ARG RELEASE=v1.3.2

# Build extension
RUN cd aggs_for_vecs && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
