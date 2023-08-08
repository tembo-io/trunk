# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/cybertec-postgresql/ora_migrator.git

# Set project version
ARG RELEASE=RELEASE_1_0_0

# Build extension
RUN cd ora_migrator && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
