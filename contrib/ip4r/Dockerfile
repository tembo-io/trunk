ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
ARG RELEASE=2.4.2

# Clone repository
RUN git clone https://github.com/RhodiumToad/ip4r.git

RUN cd ip4r && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
