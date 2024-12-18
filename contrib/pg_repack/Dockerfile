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
	ccache \
	libzstd-dev \
	liblz4-dev

# Extension build dependencies
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN curl -O https://api.pgxn.org/dist/${EXTENSION_NAME}/${EXTENSION_VERSION}/${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    && unzip ${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    && cd ${EXTENSION_NAME}-${EXTENSION_VERSION} && make
WORKDIR /app/${EXTENSION_NAME}-${EXTENSION_VERSION}
