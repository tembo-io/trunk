ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

RUN apt-get update && \
    apt-get install -y \
    libgeos-dev \
    libproj-dev \
    libprotobuf-c-dev \
    protobuf-c-compiler \
    libgdal-dev \
    libgsl-dev \
    libjson-c-dev

# Download and build PostGIS
RUN wget https://download.osgeo.org/postgis/source/postgis-3.5.0.tar.gz && \
    tar xvf postgis-3.5.0.tar.gz && \
    cd postgis-3.5.0 && \
    ./configure && \
    make && \
    make install

# Clone repository
RUN git clone https://github.com/MobilityDB/MobilityDB

# Set project version
ARG RELEASE=v1.2.0

# Build extension
RUN cd MobilityDB && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    mkdir build && \
    cd build && \
    cmake .. && \
    make
