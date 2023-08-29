# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    wget tar build-essential zlib1g-dev liblzo2-dev libxxhash-dev \
    libmsgpack-dev libzmq3-dev libevent-dev libmecab-dev

# Build Groonga 13.0.5
RUN wget https://packages.groonga.org/source/groonga/groonga-13.0.5.tar.gz \
    && tar xvzf groonga-13.0.5.tar.gz \
    && cd groonga-13.0.5 \
    && ./configure \
    && make -j$(grep '^processor' /proc/cpuinfo | wc -l) \
    && make install


# Set project version
ARG RELEASE=3.1.3

# Download and decompress the release's archive
RUN wget https://packages.groonga.org/source/pgroonga/pgroonga-${RELEASE}.tar.gz \
    && tar -xvzf pgroonga-${RELEASE}.tar.gz

# Build extension
RUN cd pgroonga-${RELEASE} \
    && make
