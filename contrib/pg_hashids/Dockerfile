ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
ARG RELEASE=v1.2.1

# Clone repository
RUN git clone https://github.com/iCyberon/pg_hashids.git

RUN cd pg_hashids && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
