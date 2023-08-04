ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/orafce/orafce.git

ARG RELEASE=VERSION_4_4_0

RUN cd orafce && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
