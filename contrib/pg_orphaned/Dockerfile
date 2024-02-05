# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Clone repository
RUN git clone https://github.com/bdrouvot/pg_orphaned.git

# Build extension
RUN cd pg_orphaned && \
    make
