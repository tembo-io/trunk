# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/ChenHuajun/pg_roaringbitmap.git

# Set project version
ARG RELEASE=v0.5.4

# Build extension
RUN cd pg_roaringbitmap && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
