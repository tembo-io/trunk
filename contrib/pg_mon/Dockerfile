ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=e496268
RUN git clone https://github.com/RafiaSabih/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout ${RELEASE} \
    && make
