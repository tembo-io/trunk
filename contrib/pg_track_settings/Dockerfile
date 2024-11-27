# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/rjuju/pg_track_settings.git

# Set project version
ARG RELEASE=2.1.2

# Build extension
RUN cd pg_track_settings && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
