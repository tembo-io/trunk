# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    libsodium-dev

# Clone repository
RUN git clone https://github.com/michelp/pgsodium.git

# Set project version
ARG RELEASE=v3.1.8

# Build extension
RUN cd pgsodium && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
