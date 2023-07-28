ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
	build-essential \
	libreadline-dev \
	zlib1g-dev \
	flex bison \
	libxml2-dev \
	libxslt-dev \
	libssl-dev \
	libxml2-utils \
	xsltproc \
	ccache

# Clone repository
RUN git clone https://github.com/postgres/postgres.git


ARG PG_RELEASE=REL_15_3
ARG RELEASE=v1.2-20200228

# Build extension
RUN cd postgres && \
	git fetch origin ${PG_RELEASE} && \
	git checkout ${PG_RELEASE} && \
	./configure && \
	cd contrib && \
        git clone https://github.com/pgbigm/pg_bigm.git && \
	cd pg_bigm && \
        git fetch origin ${RELEASE} && \
        git checkout ${RELEASE} && \
	make USE_PGXS=1
