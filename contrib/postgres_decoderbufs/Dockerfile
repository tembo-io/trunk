# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y libprotobuf-c-dev

# Clone and build the extension.
ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=84f30f3
RUN git clone https://github.com/debezium/postgres-decoderbufs.git ${EXTENSION_NAME} \
    && cd ${EXTENSION_NAME} \
    && git checkout ${RELEASE} \
    && make
