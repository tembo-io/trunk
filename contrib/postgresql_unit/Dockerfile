ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/df7cb/postgresql-unit.git

ARG RELEASE=7.7

RUN cd postgresql-unit && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
