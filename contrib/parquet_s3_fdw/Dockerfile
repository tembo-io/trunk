# Set the PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Install build dependencies for parquet_s3_fdw and its dependent libraries
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
    libprotobuf-dev \
    protobuf-compiler \
    libprotoc-dev \
    libboost-all-dev \
    libcurl4-openssl-dev \
    cmake \
    git \
    g++ \
    make \
    && rm -rf /var/lib/apt/lists/*

# Clone and build AWS SDK for C++
RUN git clone https://github.com/aws/aws-sdk-cpp.git && \
    cd aws-sdk-cpp && \
    git checkout 1.9.263 && \
    git submodule update --init --recursive && \
    mkdir build && cd build && \
    cmake -DBUILD_ONLY="s3;core;config;sts;cognito-identity;transfer;identity-management" -DAUTORUN_UNIT_TESTS=OFF -DCMAKE_CXX_FLAGS=-Wno-error=deprecated-declarations .. && \
    make -j$(nproc) && \
    make install

# Clone and build Apache Arrow
RUN git clone https://github.com/apache/arrow.git && \
    cd arrow && \
    git checkout apache-arrow-7.0.1 && \
    cd cpp && \
    mkdir build && cd build && \
    cmake -DARROW_PARQUET=ON -DARROW_S3=ON -DARROW_WITH_SNAPPY=ON .. && \
    make -j$(nproc) && \
    make install

# Clone and build parquet_s3_fdw
RUN git clone https://github.com/pgspider/parquet_s3_fdw.git && \
    cd parquet_s3_fdw && \
    git checkout v1.0.0 && \
    make USE_PGXS=1 install

# Clean up to keep the container slim
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* /aws-sdk-cpp /arrow /parquet_s3_fdw
