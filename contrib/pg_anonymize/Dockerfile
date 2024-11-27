# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/rjuju/pg_anonymize.git

# Build extension
RUN cd pg_anonymize && \
    make
