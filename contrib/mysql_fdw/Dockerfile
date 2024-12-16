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
	ccache \
	default-libmysqlclient-dev

# Clone and build the extension.
ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=REL-2_9_2
RUN git clone --depth 1 --branch "${RELEASE}" https://github.com/EnterpriseDB/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
	&& make USE_PGXS=1
