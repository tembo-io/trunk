ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

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

ARG EXTENSION_VERSION
ARG EXTENSION_NAME

# Build extension
RUN git clone https://gitlab.com/${EXTENSION_NAME}/${EXTENSION_NAME}.git \
	&& cd ${EXTENSION_NAME} \
	&& git checkout v${EXTENSION_VERSION} \
	&& make USE_PGXS=1
