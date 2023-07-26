ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/dimitri/prefix.git

ARG RELEASE=v1.2.9

RUN cd prefix && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
