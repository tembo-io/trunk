ARG PG_VERSION=15
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
 libeigen3-dev \
 libfreetype6-dev \
 python3-pip \
 postgresql-server-dev-15

# Install numpy with pip
RUN pip3 install numpy

# Clone repository
RUN git clone https://github.com/postgres/postgres.git

ARG PG_VERSION=15

# Build extension
RUN cd postgres && \
    git fetch origin REL_${PG_VERSION}_STABLE && \
    git checkout REL_${PG_VERSION}_STABLE && \
    ./configure && \
    cd contrib && \
    git clone https://github.com/rdkit/rdkit.git

# Build and Install RDKit
RUN cd postgres/contrib/rdkit && \
    mkdir build && \
    cd build && \
    cmake .. && \
    make
