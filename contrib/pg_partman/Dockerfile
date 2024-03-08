ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/pgpartman/pg_partman.git

ARG RELEASE=v5.0.1

RUN cd pg_partman && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
