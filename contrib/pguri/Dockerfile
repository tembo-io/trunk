# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y liburiparser-dev

# Clone and build the extension.
ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=1.20151224
RUN git clone --depth 1 --branch "${RELEASE}" https://github.com/petere/${EXTENSION_NAME}.git \
    && make -C ${EXTENSION_NAME}
