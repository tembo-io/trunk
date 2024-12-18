ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=0b85ec4
RUN git clone https://github.com/viggy28/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout ${RELEASE} \
    && make
