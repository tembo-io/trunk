ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/HypoPG/hypopg.git

ARG RELEASE=1.4.1

RUN cd hypopg && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
