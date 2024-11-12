# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    liburiparser-dev

# Clone repository
RUN git clone https://github.com/petere/pguri.git

# Set project version
ARG RELEASE=1.20151224

# Build extension
RUN cd pguri && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
