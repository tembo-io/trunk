# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/cybertec-postgresql/pg_permissions.git

# Set project version
ARG RELEASE=REL_1_1

# Build extension
RUN cd pg_permissions && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
