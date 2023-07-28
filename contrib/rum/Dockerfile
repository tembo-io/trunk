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
	ccache \
	systemtap-sdt-dev \
	python3-dev

# Clone repository
RUN git clone https://github.com/postgres/postgres.git

ARG PG_RELEASE=REL_15_3

# Build extension
RUN cd postgres && \
	git fetch origin ${PG_RELEASE} && \
	git checkout ${PG_RELEASE} && \
	./configure --with-python && \
	cd contrib && \
	git clone https://github.com/postgrespro/rum.git && \
	cd rum && \
	make USE_PGXS=1
