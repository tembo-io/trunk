# Set the PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root
ARG PG_VERSION=17

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
    python3 \
    python3-dev \
    python3-setuptools \
    python3-pip && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build the extension.
RUN git clone --depth 1 --branch PG${PG_VERSION}/v${EXTENSION_VERSION}-rc0 https://github.com/apache/${EXTENSION_NAME}.git \
	&& cd ${EXTENSION_NAME} \
	&& make
