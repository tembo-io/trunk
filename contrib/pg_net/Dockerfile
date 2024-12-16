ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Install dependencies
USER root
RUN apt-get update && apt-get install -y libcurl4-openssl-dev

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/supabase/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && make
