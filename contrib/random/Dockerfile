# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=dd2076e
RUN git clone https://github.com/tvondra/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout ${RELEASE} \
    && make
