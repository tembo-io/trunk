ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Install the dependencies
USER root
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    libboost-all-dev \
    python3-sphinx

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/${EXTENSION_NAME}/${EXTENSION_NAME}.git \
    && mkdir -p ${EXTENSION_NAME}/build  \
    && cd ${EXTENSION_NAME}/build  \
    && cmake .. \
    && make
