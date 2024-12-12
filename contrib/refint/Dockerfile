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
    && printf "MODULES = refint\nEXTENSION = refint\nDATA = \$(wildcard refint--*.sql)\nPG_CONFIG ?= pg_config\nPGXS := \$(shell \$(PG_CONFIG) --pgxs)\ninclude \$(PGXS)\n" > postgres/contrib/spi/Makefile \
    && cat postgres/contrib/spi/Makefile \
    && make -C postgres/contrib/spi USE_PGXS=1
