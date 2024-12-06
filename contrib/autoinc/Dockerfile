ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    libreadline-dev \
    zlib1g-dev \
    flex bison \
    libxml2-dev \
    libxslt-dev \
    libssl-dev \
    libxml2-utils \
    xsltproc \
    ccache

# Clone repository
RUN git clone https://github.com/postgres/postgres.git

ARG PG_RELEASE=REL_17_2

RUN cd postgres && \
    git fetch origin ${PG_RELEASE} && \
    git checkout ${PG_RELEASE} && \
    ./configure && \
    cd contrib/spi && \
    make

