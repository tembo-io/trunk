ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN git clone https://github.com/MigOpsRepos/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout v$(perl -E 'print shift =~ s/[.]0\$//gr' "${EXTENSION_VERSION}") \
    make
