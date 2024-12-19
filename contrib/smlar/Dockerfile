# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
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
# ARG EXTENSION_VERSION
ARG RELEASE=f2522d5
RUN git clone https://github.com/jirutka/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout ${RELEASE} \
    && make USE_PGXS=1
