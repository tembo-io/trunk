# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    autoconf \
    automake \
    libtool \
    zlib1g-dev \
    libxml2-dev \
    libcunit1-dev

# Clone repository
RUN git clone https://github.com/pgpointcloud/pointcloud.git

# Set project version
ARG RELEASE=v1.2.5

# Build extension
RUN cd pointcloud && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    ./autogen.sh && \
    ./configure && \
    make
