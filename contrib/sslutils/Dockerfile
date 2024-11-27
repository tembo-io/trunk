ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/EnterpriseDB/sslutils.git

ARG RELEASE=v1.3

RUN cd sslutils && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
