# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/enova/pgl_ddl_deploy.git

# Set project version
ARG RELEASE=v2.1.0

# Build extension
RUN cd pgl_ddl_deploy && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
