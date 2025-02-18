ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Install libduckdb and create the funky name that duckdb_fdw's Makefile wants at compile time.
USER root
ENV DUCKDB_VERSION=1.2.0
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
    # Download and install libduckdb.so
RUN curl -LO https://github.com/duckdb/duckdb/releases/download/v${DUCKDB_VERSION}/libduckdb-linux-amd64.zip \
    && unzip libduckdb-linux-amd64.zip libduckdb.so \
    && mv libduckdb.so /usr/local/lib/libduckdb.so.${DUCKDB_VERSION} \
    && ldconfig \
    # Create the silly file name the Makefile wants.
    && (cd /usr/local/lib && ln -s "libduckdb.so.${DUCKDB_VERSION}" libduckdb.${DUCKDB_VERSION}.so) \
    # Clone the repo, remove the useless `install` target, and build
    && git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/alitrack/${EXTENSION_NAME}.git \
    && perl -i -pe 's/^install:/#/' "${EXTENSION_NAME}/Makefile" \
    && make -C ${EXTENSION_NAME} USE_PGXS=1
