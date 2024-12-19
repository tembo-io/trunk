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
ARG PG_VERSION
RUN git clone --depth 1 --branch "REL_$(perl -E 'print shift =~ s/[.]/_/gr' "${EXTENSION_VERSION}")" https://github.com/${EXTENSION_NAME}-team/${EXTENSION_NAME}-archivist.git ${EXTENSION_NAME} \
    && make -C ${EXTENSION_NAME}
