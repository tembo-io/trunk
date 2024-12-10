ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN git clone https://github.com/df7cb/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout $(perl -E 'print shift =~ s/[.]0\$//gr' "${EXTENSION_VERSION}") \
    make
