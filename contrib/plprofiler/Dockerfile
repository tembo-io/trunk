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

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "REL$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}")" https://github.com/bigsql/${EXTENSION_NAME}.git \
    && make -C ${EXTENSION_NAME} USE_PGXS=1
