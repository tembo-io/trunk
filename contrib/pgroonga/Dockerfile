# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y \
    wget tar build-essential zlib1g-dev liblzo2-dev libxxhash-dev \
    libmsgpack-dev libzmq3-dev libevent-dev libmecab-dev

# Build Groonga 14.1.2
RUN curl -LO https://packages.groonga.org/source/groonga/groonga-14.1.2.tar.gz \
    && tar xvzf groonga-14.1.2.tar.gz \
    && cd groonga-14.1.2 \
    && ./configure \
    && make -j$(grep '^processor' /proc/cpuinfo | wc -l) \
    && make install

# Download and build extension
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN curl -LO https://github.com/${EXTENSION_NAME}/${EXTENSION_NAME}/releases/download/${EXTENSION_VERSION}/${EXTENSION_NAME}-${EXTENSION_VERSION}.tar.gz \
    && tar zxf ${EXTENSION_NAME}-${EXTENSION_VERSION}.tar.gz \
    && make -C ${EXTENSION_NAME}-${EXTENSION_VERSION}
WORKDIR /app/${EXTENSION_NAME}-${EXTENSION_VERSION}
