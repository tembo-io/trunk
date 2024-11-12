# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    bison

# Clone repository
RUN git clone https://github.com/plproxy/plproxy.git

# Set project version
ARG RELEASE=v2.10.0

# Build extension
RUN cd plproxy && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
