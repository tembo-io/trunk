ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

RUN apt-get update \
	&& apt-get install -y \
		libgeos-dev \
		libproj-dev \
		libprotobuf-c-dev \
		protobuf-c-compiler \
		libgdal-dev \
	&& wget https://download.osgeo.org/postgis/source/postgis-3.4.0.tar.gz \
	&& tar xvf postgis-3.4.0.tar.gz \
	&& cd postgis-3.4.0 \
	&& ./configure \
	&& make
