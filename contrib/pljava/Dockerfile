# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    openjdk-11-jdk \
    maven \
    libkrb5-dev \
    libecpg-dev

# Clone repository
RUN git clone https://github.com/tada/pljava.git

# Set project version
ARG RELEASE=V1_6_5

# Build extension
RUN cd pljava && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    mvn clean install
