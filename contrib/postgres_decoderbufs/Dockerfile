# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    libprotobuf-c-dev

# Clone repository
RUN git clone https://github.com/debezium/postgres-decoderbufs.git

# Build extension
RUN cd postgres-decoderbufs && \
    make
