# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y liburiparser-dev

# Clone and build the extension.
ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=1.20151224
RUN git clone --depth 1 --branch "${RELEASE}" https://github.com/petere/${EXTENSION_NAME}.git \
    # Patch for Postgres 16+ https://github.com/petere/pguri/issues/16
    && perl -i -pe 's/^(PG_CPPFLAGS.+)/$1 -Wno-int-conversion/' ${EXTENSION_NAME}/Makefile \
    && make -C ${EXTENSION_NAME}
