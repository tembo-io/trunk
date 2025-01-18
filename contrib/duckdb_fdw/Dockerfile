ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Download and install the DuckDB DSO.
ARG EXTENSION_VERSION
ENV DUCKDB_VERSION=${EXTENSION_VERSION}
USER root
# Funky naming expectations: https://github.com/alitrack/duckdb_fdw/issues/64
RUN curl -LO https://github.com/duckdb/duckdb/releases/download/v${DUCKDB_VERSION}/libduckdb-linux-amd64.zip \
    && unzip -d . libduckdb-linux-amd64.zip \
    && install -m 755 libduckdb.so "/usr/local/lib/libduckdb.${DUCKDB_VERSION}.so" \
    && (cd /usr/local/lib && ln -s "libduckdb.${DUCKDB_VERSION}.so" "libduckdb.so")

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/alitrack/${EXTENSION_NAME}.git \
    && perl -i -pe 's/^install:/#/' "${EXTENSION_NAME}/Makefile" \
    && make -C ${EXTENSION_NAME} USE_PGXS=1
