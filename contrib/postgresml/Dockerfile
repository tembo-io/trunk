ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
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
	python3-dev \
	clang \
	cmake \
	libclang-dev \
	libopenblas-dev \
	libpython3-dev \
	pkg-config

# Clone repository
RUN git clone https://github.com/postgresml/postgresml.git

ARG RELEASE=v2.7.0

RUN cd postgresml && \
	git submodule update --init --recursive && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
	cd pgml-extension

#	./configure --with-python && \
#	cd contrib/postgresml && \
#	make
