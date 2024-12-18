ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
# ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN curl -O https://api.pgxn.org/dist/semver/${EXTENSION_VERSION}/semver-${EXTENSION_VERSION}.zip \
    && unzip semver-${EXTENSION_VERSION}.zip \
    && cd semver-${EXTENSION_VERSION} && make
WORKDIR /app/semver-${EXTENSION_VERSION}
