ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/sraoss/pg_ivm.git

ARG RELEASE=v1.5.1

RUN cd pg_ivm && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
