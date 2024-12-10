ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG RELEASE=v1.5

RUN git clone https://github.com/MigOpsRepos/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout ${RELEASE} \
    make
