ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/aws/pg_tle.git

ARG RELEASE=v1.0.4

RUN cd pg_tle && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
