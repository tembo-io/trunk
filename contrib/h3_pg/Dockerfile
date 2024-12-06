# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    cmake

# Clone repository
RUN git clone https://github.com/zachasme/h3-pg.git

# Set project version
ARG RELEASE=v4.1.4

# Build extension
RUN cd h3-pg && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
