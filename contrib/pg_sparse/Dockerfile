# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

ARG RELEASE=v0.4.2

# Clone repository
RUN git clone https://github.com/paradedb/paradedb.git && \
	cd paradedb && \
    git fetch --tags && \
    git fetch --depth 1 origin tag ${RELEASE} && \
    git checkout ${RELEASE}

RUN cd paradedb/pg_sparse && \
    make USE_PGXS=1 OPTFLAGS="" && \
    make install
