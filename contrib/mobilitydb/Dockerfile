ARG PG_VERSION
# Set up image to copy trunk from.
FROM quay.io/tembo/trunk-test-tembo:73092d1-pg15 AS trunk
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

USER root
RUN apt-get update && \
    apt-get install -y \
    libgeos-dev \
    libproj-dev \
    libprotobuf-c-dev \
    protobuf-c-compiler \
    libgdal-dev \
    libgsl-dev \
    libjson-c-dev

# Install trunk
COPY --from=trunk /usr/bin/trunk /usr/bin/trunk

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN trunk install postgis --version 3.5.0 \
    && git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/${EXTENSION_NAME}/${EXTENSION_NAME}.git \
    && cmake -S ${EXTENSION_NAME} -B ${EXTENSION_NAME}/build \
    && make -C ${EXTENSION_NAME}/build -j8
