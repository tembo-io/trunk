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

# Download and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN curl -O https://api.pgxn.org/dist/${EXTENSION_NAME}/${EXTENSION_VERSION}/${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    && unzip ${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    && cd ${EXTENSION_NAME}-${EXTENSION_VERSION} && make USE_PGXS=1
WORKDIR /app/${EXTENSION_NAME}-${EXTENSION_VERSION}
