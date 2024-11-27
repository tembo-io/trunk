ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/klando/pgfincore.git

ARG RELEASE=1.2.4

RUN cd pgfincore && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
