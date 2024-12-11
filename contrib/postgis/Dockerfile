ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update apt-get install -y \
	libgeos-dev \
	libproj-dev \
	libprotobuf-c-dev \
	protobuf-c-compiler \
	libgdal-dev

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN wget https://download.osgeo.org/${EXTENSION_NAME}/source/${EXTENSION_NAME}-${EXTENSION_VERSION}.tar.gz \
	&& tar xvf ${EXTENSION_NAME}-${EXTENSION_VERSION}.tar.gz \
	&& cd ${EXTENSION_NAME}-${EXTENSION_VERSION} \
	&& ./configure \
	&& make
