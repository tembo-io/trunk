ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/fboulnois/pg_uuidv7.git

ARG RELEASE=v1.0.2

RUN cd pg_uuidv7 && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
