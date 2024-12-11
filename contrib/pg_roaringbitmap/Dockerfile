# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build extension
RUN git clone https://github.com/ChenHuajun/pg_roaringbitmap.git \
    && cd pg_roaringbitmap \
    && git checkout v${EXTENSION_VERSION} \
    && make
