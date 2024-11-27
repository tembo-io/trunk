ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/theory/pg-semver.git

ARG RELEASE=v0.32.1

RUN cd pg-semver && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
