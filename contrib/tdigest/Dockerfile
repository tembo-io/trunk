ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/tvondra/tdigest.git

ARG RELEASE=v1.4.0

RUN cd tdigest && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
