# Set PostgreSQL version
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

# Clone postgres repository
ARG PG_RELEASE=REL_15_3
RUN git clone  --depth 1 --branch ${PG_RELEASE} https://github.com/postgres/postgres.git
RUN cd postgres && \
	./configure

#Clone jdbc repository
RUN cd postgres/contrib && \
	git clone https://github.com/jirutka/smlar.git && \
	cd smlar && git checkout f2522d5f20a46a3605a761d34a3aefcdffb94e71

RUN cd postgres/contrib/smlar && \
    make clean && \
    make
