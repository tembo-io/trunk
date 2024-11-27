ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/xocolatl/periods.git

ARG RELEASE=v1.2.2

RUN cd periods && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
