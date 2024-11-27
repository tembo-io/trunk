ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    libcurl4-openssl-dev

# Clone repository
RUN git clone https://github.com/pramsey/pgsql-http.git

ARG RELEASE=v1.5.0

RUN cd pgsql-http && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
