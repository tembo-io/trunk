# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Clone repository
RUN git clone https://github.com/bigsmoke/pg_extra_time.git

ARG RELEASE=v1.1.0

# Build extension
RUN cd pg_extra_time && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make

