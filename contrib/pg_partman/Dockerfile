ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/pgpartman/pg_partman.git

ARG EXTENSION_VERSION

RUN cd pg_partman && \
    git fetch origin v${EXTENSION_VERSION} && \
    git checkout v${EXTENSION_VERSION} && \
    make
