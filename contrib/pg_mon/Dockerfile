ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/RafiaSabih/pg_mon.git

ARG RELEASE=master

RUN cd pg_mon && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
