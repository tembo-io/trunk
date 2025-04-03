ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

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
    cmake \
    gcc \
    libboost-all-dev \
    libfreetype6-dev

# Build and Install RDKit
ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=Release_2025_03_1
RUN curl -LO https://github.com/${EXTENSION_NAME}/${EXTENSION_NAME}/archive/${RELEASE}.tar.gz \
    && tar xvf ${RELEASE}.tar.gz \
    && cd ${EXTENSION_NAME}-${RELEASE} \
    && cmake -B build \
        -D RDK_BUILD_PGSQL=ON \
        -D RDK_BUILD_PYTHON_WRAPPERS=OFF \
        -D PostgreSQL_CONFIG_DIR=/usr/bin \
        -D "PostgreSQL_INCLUDE_DIR=$(pg_config --includedir)" \
        -D "PostgreSQL_TYPE_INCLUDE_DIR=$(pg_config --includedir-server)" \
    && make -j8 -C build
WORKDIR /app/${EXTENSION_NAME}-${RELEASE}
