# Set the PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Install build dependencies for parquet_s3_fdw and its dependent libraries
USER root
RUN apt-get update && apt-get install -y \
    automake \
    bison \
    build-essential \
    ccache \
    cmake \
    flex \
    g++ \
    git \
    libboost-all-dev \
    libcurl4-openssl-dev \
    libevent-dev \
    libprotobuf-dev \
    libprotoc-dev \
    libreadline-dev \
    libssl-dev \
    libtool \
    libxml2-dev \
    libxml2-utils \
    libxslt-dev \
    make \
    pkg-config \
    protobuf-compiler \
    xsltproc \
    zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

# Clone and build AWS SDK for C++
RUN git clone --depth 1 --branch 1.11.470 https://github.com/aws/aws-sdk-cpp.git \
    && cd aws-sdk-cpp \
    && git submodule update --init --recursive \
    && mkdir build && cd build \
    && cmake -DBUILD_ONLY="s3;core;config;sts;cognito-identity;transfer;identity-management" -DAUTORUN_UNIT_TESTS=OFF -DCMAKE_CXX_FLAGS=-Wno-error=deprecated-declarations .. \
    && make -j$(nproc) \
    && make install \
    && cd ../../ && rm -rf aws-sdk-cpp

# Clone and build Apache Arrow
RUN git clone --depth 1 --branch apache-arrow-18.1.0 https://github.com/apache/arrow.git \
    && mkdir -p arrow/cpp/build \
    && cd arrow/cpp/build \
    && cmake -DARROW_PARQUET=ON -DARROW_S3=ON -DARROW_WITH_SNAPPY=ON .. \
    && make -j$(nproc) \
    && make install \
    && cd ../../ && rm -rf arrow

# Clone and build parquet_s3_fdw
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/pgspider/${EXTENSION_NAME}.git \
    && make -C "${EXTENSION_NAME}" USE_PGXS=1
