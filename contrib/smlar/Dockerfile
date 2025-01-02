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
		ccache

ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=6259352
RUN curl -o "${EXTENSION_NAME}-${RELEASE}.tgz" "http://sigaev.ru/git/gitweb.cgi?p=${EXTENSION_NAME}.git;a=snapshot;h=${RELEASE};sf=tgz" \
	&& tar zxf "${EXTENSION_NAME}-${RELEASE}.tgz" \
    && cd "${EXTENSION_NAME}-${RELEASE}" \
    && make USE_PGXS=1
WORKDIR /app/${EXTENSION_NAME}-${RELEASE}
