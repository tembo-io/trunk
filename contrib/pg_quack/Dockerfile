ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Clone repository
RUN git clone https://github.com/hydradatabase/pg_quack.git

ARG RELEASE=v0.0.1

RUN cd pg_quack &&  \
    git checkout ${RELEASE} &&  \
    make

