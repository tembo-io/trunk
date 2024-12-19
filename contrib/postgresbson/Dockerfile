ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y libbson-dev

# Extension build dependencies
# ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN curl -O https://api.pgxn.org/dist/bson/${EXTENSION_VERSION}/bson-${EXTENSION_VERSION}.zip \
    && unzip bson-${EXTENSION_VERSION}.zip \
    && cd bson-${EXTENSION_VERSION} \
    # For some reason these need to be in ~/projects/bson/include/ and ./
    && mkdir -p ~/projects/bson \
    && cp -r /usr/include/libbson-1.0 ~/projects/bson/include \
    && cp -r /usr/include/libbson-1.0/* ./ \
    && make BSON_SHLIB=/usr/lib/x86_64-linux-gnu/libbson-1.0.so
WORKDIR /app/bson-${EXTENSION_VERSION}
