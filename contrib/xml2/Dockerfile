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

	# Clone repository and build extension.
ARG EXTENSION_NAME
ARG PG_RELEASE
RUN git clone --depth 1 --branch "${PG_RELEASE}" https://github.com/postgres/postgres.git \
	&& make -C postgres/contrib/${EXTENSION_NAME} USE_PGXS=1
