# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Clone repository
RUN git clone https://github.com/dverite/icu_ext.git

ARG RELEASE=v1.8.0

# Build extension
RUN cd icu_ext && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
