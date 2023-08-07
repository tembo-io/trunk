# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    libgdal-dev

# Clone repository
RUN git clone https://github.com/pramsey/pgsql-ogr-fdw.git

# Set project version
ARG RELEASE=v1.1.4

# Build extension
RUN cd pgsql-ogr-fdw && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
