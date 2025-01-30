ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Install build dependencies.
USER root
RUN apt-get update && apt-get install -y libsqlite3-dev

# Download and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN curl -O "https://api.pgxn.org/dist/${EXTENSION_NAME}/${EXTENSION_VERSION}/${EXTENSION_NAME}-${EXTENSION_VERSION}.zip" \
    && unzip "${EXTENSION_NAME}-${EXTENSION_VERSION}.zip" \
    && make -C "${EXTENSION_NAME}-${EXTENSION_VERSION}" USE_PGXS=1
WORKDIR "/app/${EXTENSION_NAME}-${EXTENSION_VERSION}"
