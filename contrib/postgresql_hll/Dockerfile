ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/citusdata/postgresql-hll.git

ARG RELEASE=v2.17

RUN cd postgresql-hll && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
