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

ARG EXTENSION_NAME
ARG RELEASE=v1.2-20200228

# Build extension
RUN git clone https://github.com/pgbigm/${EXTENSION_NAME}.git \
	&& cd ${EXTENSION_NAME} \
    && git checkout ${RELEASE} \
	make USE_PGXS=1
