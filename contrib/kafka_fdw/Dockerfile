# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y  librdkafka-dev zlib1g-dev

ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=c73ab18
RUN git clone https://github.com/adjust/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout ${RELEASE} \
    && make
