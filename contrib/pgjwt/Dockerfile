ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG REVISION=f3d82fd

RUN git clone --depth 1 --branch ${REVISION} https://github.com/michelp/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} && \
    && make
