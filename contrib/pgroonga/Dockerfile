# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    wget tar build-essential zlib1g-dev liblzo2-dev libxxhash-dev \
    libmsgpack-dev libzmq3-dev libevent-dev libmecab-dev software-properties-common libgroonga-dev

RUN add-apt-repository -y universe \
    && add-apt-repository -y ppa:groonga/ppa \
    && apt-get update \
    && apt-get -y install groonga

# Set project version
ARG RELEASE=3.1.3

# Download and decompress the release's archive
RUN wget https://packages.groonga.org/source/pgroonga/pgroonga-${RELEASE}.tar.gz \
    && tar -xvzf pgroonga-${RELEASE}.tar.gz

# Build extension
RUN cd pgroonga-${RELEASE} \
    && make
