ARG PG_VERSION=15
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
	ccache

# Clone repository
RUN git clone https://github.com/pgvector/pgvector.git

ARG RELEASE=v0.4.4

# Build extension
RUN cd pgvector && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
	make
