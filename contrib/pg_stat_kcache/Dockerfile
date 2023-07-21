ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/powa-team/pg_stat_kcache.git

ARG RELEASE=REL2_2_1

RUN cd pg_stat_kcache && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
