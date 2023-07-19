ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    libkrb5-dev

# Clone repository
RUN git clone https://github.com/MigOpsRepos/credcheck.git

ARG RELEASE=v2.0

RUN cd credcheck && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
