ARG PG_VERSION=15
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
ARG EXTENSION_VERSION

RUN git clone --depth 1 --branch  $(perl -E 'print shift =~ s/[.]0\$//gr' "${EXTENSION_VERSION}") https://github.com/ossc-db/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} && \
    && make
