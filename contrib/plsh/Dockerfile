# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=1.20220917
RUN git clone --depth 1 --branch "${RELEASE}" https://github.com/petere/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && make
