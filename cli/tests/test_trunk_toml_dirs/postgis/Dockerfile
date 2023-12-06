ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

RUN apt-get update && apt-get install -y \
 build-essential \
 libreadline-dev \
 zlib1g-dev \
 flex \
 bison \
 libxml2-dev \
 libxslt-dev \
 libssl-dev \
 libxml2-utils \
 xsltproc \
 ccache \
 libgdal-dev \
 libgeos-dev \
 libprotobuf-c-dev \
 protobuf-c-compiler

RUN wget https://download.osgeo.org/postgis/source/postgis-3.4.0.tar.gz && \
	tar xvf postgis-3.4.0.tar.gz && \
	cd postgis-3.4.0 && \
	./configure && \
	make -j$(nproc)