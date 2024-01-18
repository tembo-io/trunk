ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/df7cb/pg_dirtyread.git

ARG RELEASE=2.6

RUN cd pg_dirtyread && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
