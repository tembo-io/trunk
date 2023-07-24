ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/aquameta/meta.git

ARG RELEASE=v0.3.0

RUN cd meta && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
