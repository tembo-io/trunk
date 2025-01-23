ARG PG_VERSION=17
# Set up image to copy libduckdb from.
FROM quay.io/tembo/standard-cnpg:${PG_VERSION}-f9c2075 AS std
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Copy the DuckDB DSO.
USER root
COPY --from=std "/usr/local/lib/libduckdb.*.so" /usr/local/lib
RUN cd /usr/local/lib && ln -s "libduckdb.*.so" "libduckdb.so"

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
ENV DUCKDB_VERSION=1.1.3
RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/alitrack/${EXTENSION_NAME}.git \
    && perl -i -pe 's/^install:/#/' "${EXTENSION_NAME}/Makefile" \
    && make -C ${EXTENSION_NAME} USE_PGXS=1
