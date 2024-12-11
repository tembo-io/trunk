# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build extension
RUN git clone https://github.com/begriffs/pg_rational.git \
    && cd pg_rational \
    && git checkout v${EXTENSION_VERSION} \
    && make
