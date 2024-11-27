ARG PG_VERSION=17
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
 libfreetype6-dev \
 postgresql-server-dev-all

ARG RELEASE=Release_2023_03_2
ARG RDBASE=rdkit-${RELEASE}

# Build and Install RDKit
RUN wget https://github.com/rdkit/rdkit/archive/${RELEASE}.tar.gz && \
    tar xvf ${RELEASE}.tar.gz && \
    cd ${RDBASE} && \
    mkdir build && \
    cd build && \
    cmake -DRDK_BUILD_PGSQL=ON \
    -DRDK_BUILD_PYTHON_WRAPPERS=OFF \
    -DRDK_BUILD_CPP_TESTS=OFF \
    -DPostgreSQL_CONFIG_DIR=/usr/bin .. && \
    make -j8
