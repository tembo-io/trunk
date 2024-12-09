ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Set project version
ARG RELEASE=1.1.3

RUN git clone https://github.com/alitrack/duckdb_fdw \
    && cd duckdb_fdw \
    && wget -c https://github.com/duckdb/duckdb/releases/download/v${RELEASE}/libduckdb-linux-amd64.zip \
    && unzip libduckdb-linux-amd64.zip libduckdb.so \
    && cp libduckdb.so $(pg_config --libdir) \
    && make USE_PGXS=1
