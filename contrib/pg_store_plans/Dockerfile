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

# Use argument to specify PostgreSQL release
ARG PG_RELEASE=REL_15_3

# Configure and build PostgreSQL
RUN cd postgres && \
    git fetch origin ${PG_RELEASE} && \
    git checkout ${PG_RELEASE} && \
    ./configure && \
    cd contrib && \
    git clone https://github.com/ossc-db/pg_store_plans.git

ARG RELEASE=1.7

# Build extension
RUN cd postgres/contrib/pg_store_plans && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
