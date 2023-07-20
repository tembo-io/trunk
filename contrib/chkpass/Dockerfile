ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/lacanoid/chkpass.git

ARG RELEASE=master

RUN cd chkpass && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
