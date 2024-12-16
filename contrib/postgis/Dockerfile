ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

USER root
RUN apt-get update && apt-get install -y \
	libgeos-dev \
	libproj-dev \
	libprotobuf-c-dev \
	protobuf-c-compiler \
	libgdal-dev

# Extension build dependencies
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN curl -O https://download.osgeo.org/${EXTENSION_NAME}/source/${EXTENSION_NAME}-${EXTENSION_VERSION}.tar.gz \
	&& tar xvf ${EXTENSION_NAME}-${EXTENSION_VERSION}.tar.gz \
	&& cd ${EXTENSION_NAME}-${EXTENSION_VERSION} \
	&& ./configure \
	&& make
WORKDIR /app/${EXTENSION_NAME}-${EXTENSION_VERSION}
