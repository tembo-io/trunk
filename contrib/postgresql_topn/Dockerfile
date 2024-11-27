ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/citusdata/postgresql-topn.git

ARG RELEASE=v2.4.0

RUN cd postgresql-topn && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
