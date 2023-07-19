ARG PG_VERSION=14
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
# RUN apt-get update && apt-get install -y \
#     build-essential \
#     libreadline-dev \
#     zlib1g-dev \
#     flex bison \
#     libxml2-dev \
#     libxslt-dev \
#     libssl-dev \
#     libxml2-utils \
#     xsltproc \
#     ccache

# Clone repository
RUN git clone https://github.com/hydradatabase/hydra

ARG PG_VERSION=14

RUN cd hydra/columnar &&  \
    git checkout 0.3.0-alpha &&  \
    ./configure && \
    make

