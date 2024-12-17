ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Extension build dependencies
RUN curl -O https://api.pgxn.org/dist/currency/${EXTENSION_VERSION}/currency-${EXTENSION_VERSION}.zip \
    && unzip currency-${EXTENSION_VERSION}.zip \
    && cd currency-${EXTENSION_VERSION} && make
WORKDIR /app/currency-${EXTENSION_VERSION}
