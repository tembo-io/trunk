# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y \
    cmake \
    libcurl4-openssl-dev \
    uuid-dev

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "${EXTENSION_VERSION}" https://github.com/ildus/${EXTENSION_NAME}.git \
    && cmake -S ${EXTENSION_NAME} -B ${EXTENSION_NAME}/build \
    && make -C ${EXTENSION_NAME}/build -j8
