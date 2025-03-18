# Set PostgreSQL version
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
		ccache \
        openjdk-11-jdk

# Clone and build the extension.
ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=v0.5.0
RUN git clone --depth 1 --branch "${RELEASE}" https://github.com/pgspider/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
	&& mkdir -p /usr/lib64 \
	&& ln -s $(find /usr/lib/jvm/ -name 'libjvm.so') /usr/lib64/ \
	&& make clean USE_PGXS=1 \
    && make USE_PGXS=1
