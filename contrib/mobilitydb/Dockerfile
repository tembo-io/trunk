ARG PG_VERSION=15
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

# Set the library path
# ENV LD_LIBRARY_PATH=/usr/lib/postgresql/15/lib:/usr/lib/x86_64-linux-gnu:$LD_LIBRARY_PATH

# Download and build PostGIS
RUN wget https://download.osgeo.org/postgis/source/postgis-3.4.0.tar.gz && \
    tar xvf postgis-3.4.0.tar.gz && \
    cd postgis-3.4.0 && \
    ./configure && \
    make && \
    make install

# # Clone repository
# RUN git clone https://github.com/MobilityDB/MobilityDB
#
# # Set project version
# ARG RELEASE=v1.0

RUN wget -O mobilitydb-1.0.tar.gz https://github.com/MobilityDB/MobilityDB/archive/v1.0.tar.gz && \
    tar xvfz mobilitydb-1.0.tar.gz && \
    ls -l

# Build extension
RUN cd MobilityDB-1.0 && \
    # git fetch origin ${RELEASE} && \
    # git checkout ${RELEASE} && \
    mkdir build && \
    cd build && \
    cmake .. && \
    make
