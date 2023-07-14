ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Install the dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    libboost-all-dev \
    perl \
    postgresql-server-dev-13 \
    python3-sphinx

# Clone pgRouting repository
RUN git clone https://github.com/pgRouting/pgrouting.git

RUN cd pgrouting \
    && mkdir build \
    && cd build \
    && cmake .. \
    && make
