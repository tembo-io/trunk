ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

RUN git clone https://github.com/alitrack/duckdb_fdw 


RUN cd duckdb_fdw && make USE_PGXS=1

