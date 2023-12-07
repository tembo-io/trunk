# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Clone repository
RUN git clone https://github.com/dverite/permuteseq.git

ARG RELEASE=v1.2.2

# Build extension
RUN cd permuteseq && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make

