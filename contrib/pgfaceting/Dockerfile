# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    cmake

# Clone repository
RUN git clone https://github.com/cybertec-postgresql/pgfaceting.git

# Set project version
ARG RELEASE=v0.2.0

# Build extension
RUN cd pgfaceting && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
