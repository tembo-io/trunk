ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y libkrb5-dev

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN git clone https://github.com/EnterpriseDB/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME}\
    && git checkout v${EXTENSION_VERSION} \
    && make
