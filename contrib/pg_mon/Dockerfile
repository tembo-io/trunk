ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
#USER root

# Extension build dependencies
#RUN apt-get update && apt-get install -y \
#    libkrb5-dev

# Clone repository
RUN git clone https://github.com/RafiaSabih/pg_mon.git

ARG RELEASE=master

RUN cd pg_mon && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
