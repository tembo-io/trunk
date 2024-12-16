# Set the PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
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
    python3 \
    python3-dev \
    python3-setuptools \
    python3-pip && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN RELEASE="$(perl -e 'print shift =~ s/[.]0$//r' "${EXTENSION_VERSION}")" \
    && wget "https://github.com/pgsql-io/${EXTENSION_NAME}2/archive/refs/tags/v${RELEASE}.tar.gz" \
    && tar -xvf "v${RELEASE}.tar.gz" \
    && mv "${EXTENSION_NAME}2-${RELEASE}" "${EXTENSION_NAME}" \
    && make -C "${EXTENSION_NAME}"
