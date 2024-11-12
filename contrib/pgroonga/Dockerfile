# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    wget tar build-essential zlib1g-dev liblzo2-dev libxxhash-dev \
    libmsgpack-dev libzmq3-dev libevent-dev libmecab-dev

# Build Groonga 13.0.4
RUN wget https://packages.groonga.org/source/groonga/groonga-13.0.4.tar.gz \
    && tar xvzf groonga-13.0.4.tar.gz \
    && cd groonga-13.0.4 \
    && ./configure \
    && make -j$(grep '^processor' /proc/cpuinfo | wc -l) \
    && make install

# Download and decompress the release's archive
RUN curl -L -o pgroonga.tar.gz https://github.com/pgroonga/pgroonga/archive/refs/tags/3.1.1.tar.gz \
    && mkdir pgroonga \
    && tar -xzvf pgroonga.tar.gz -C pgroonga --strip-components=1 \
    && cd pgroonga/vendor/ \
    && git clone https://github.com/Cyan4973/xxHash

# Set project version
ARG RELEASE=3.1.1

# Build extension
RUN cd pgroonga \
    && make
