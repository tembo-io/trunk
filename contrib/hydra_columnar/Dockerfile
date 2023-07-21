ARG PG_VERSION=14
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
      lsb-release gcc make libssl-dev autoconf pkg-config \
      libcurl4-gnutls-dev \
      liblz4-dev libzstd-dev 

# Clone repository
RUN git clone https://github.com/hydradatabase/hydra

RUN cd hydra/columnar &&  \
    git checkout 0.3.0-alpha &&  \
    ./configure && \
    make

