ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
ARG RELEASE=wal2json_2_5

# Clone repository
RUN git clone https://github.com/eulerto/wal2json.git

RUN cd wal2json && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
