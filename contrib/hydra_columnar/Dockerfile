ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
 RUN apt-get update && apt-get install -y \
     build-essential \
     libcurl4-openssl-dev \
     liblz4-dev \
     libzstd-dev

# Clone repository
RUN git clone https://github.com/hydradatabase/hydra

ARG RELEASE=v1.0.0-beta

RUN cd hydra/columnar &&  \
    git checkout ${RELEASE} &&  \
    ./configure && \
    make

