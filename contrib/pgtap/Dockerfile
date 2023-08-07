ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/theory/pgtap.git

ARG RELEASE=v1.2.0

RUN cd pgtap && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
