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
	default-libmysqlclient-dev

ARG PG_RELEASE=REL_17_2
ARG EXTENSION_VERSION

# Build extension
RUN git clone https://github.com/EnterpriseDB/mysql_fdw.git \
	&& cd mysql_fdw \
	&& git checkout REL-$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}") \
	&& make USE_PGXS=1
