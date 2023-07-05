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

ARG PG_VERSION=15

RUN cd postgres && \
	git fetch origin REL_${PG_VERSION}_STABLE && \
	git checkout REL_${PG_VERSION}_STABLE && \
	./configure && \
	cd contrib/pg_buffercache && \
	make