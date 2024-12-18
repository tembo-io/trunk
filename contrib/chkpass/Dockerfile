ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=87159fd
RUN git clone https://github.com/lacanoid/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout ${RELEASE} \
    && make
