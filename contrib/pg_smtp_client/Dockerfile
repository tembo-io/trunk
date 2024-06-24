ARG PG_VERSION=16
ARG PGRX_VERSION=0.11.4

FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx${PGRX_VERSION}

ARG EXTENSION_VERSION=0.1.0

RUN git clone --depth 1 --branch v${EXTENSION_VERSION} https://github.com/brianpursley/pg_smtp_client && \
    cd pg_smtp_client && \
    cargo pgrx package
