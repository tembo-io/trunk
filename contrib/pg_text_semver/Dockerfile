# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/bigsmoke/pg_text_semver.git

# Set project version
ARG RELEASE=v0.1.4

# Build extension
RUN cd pg_text_semver && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
