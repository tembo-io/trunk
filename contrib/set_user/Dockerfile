# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/pgaudit/set_user.git

# Set project version
ARG RELEASE=REL4_0_1

# Build extension
RUN cd set_user && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
