ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Extension build dependencies
RUN curl -O https://api.pgxn.org/dist/base62/${EXTENSION_VERSION}/base62-${EXTENSION_VERSION}.zip \
    && unzip base62-${EXTENSION_VERSION}.zip \
    && cd base62-${EXTENSION_VERSION} && make
WORKDIR /app/base62-${EXTENSION_VERSION}
