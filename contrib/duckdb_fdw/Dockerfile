ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

RUN git clone https://github.com/alitrack/duckdb_fdw 
RUN cd duckdb_fdw && \
	wget -c https://github.com/duckdb/duckdb/releases/download/v0.8.1/libduckdb-linux-amd64.zip && \
 	unzip -d . libduckdb-linux-amd64.zip && \
 	cp libduckdb.so $(pg_config --libdir)

RUN cd duckdb_fdw && make USE_PGXS=1

