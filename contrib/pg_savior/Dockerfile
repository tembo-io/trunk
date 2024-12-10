ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION
ARG REVISION=0b85ec4

RUN git clone https://github.com/viggy28/pg_savior \
    && cd pg_savior \
    && git checkout ${REVISION} \
    && make
