ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/pghydro/pghydro.git

ARG RELEASE=v.6.6

# Build extension
RUN cd pghydro && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE}
