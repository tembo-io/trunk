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
    # https://github.com/pgRouting/pgrouting/pull/2728/files
    && perl -i -pe 's/cmake_policy\(SET CMP0148 OLD\)/if(POLICY CMP0148)\n    cmake_policy(SET CMP0148 OLD)\nendif()/' ${EXTENSION_NAME}/CMakeLists.txt \
    && cmake -S ${EXTENSION_NAME} -B ${EXTENSION_NAME}/build \
    && make -C ${EXTENSION_NAME}/build -j8
