# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    r-base-dev

# Clone repository
RUN git clone https://github.com/postgres-plr/plr.git

# Set project version
ARG RELEASE=REL8_4_5

# Build extension
RUN cd plr && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make USE_PGXS=1
