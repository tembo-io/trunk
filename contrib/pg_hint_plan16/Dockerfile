ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN git clone https://github.com/ossc-db/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout REL16_$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}") \
    make
