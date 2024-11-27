ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

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
	ccache \
	libtinfo5 \
	build-essential \
	ninja-build \
	python3 \
   	pkg-config

# Clone repository
RUN git clone https://github.com/plv8/plv8.git

ARG RELEASE=v3.1.10

RUN cd plv8 && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
	make
