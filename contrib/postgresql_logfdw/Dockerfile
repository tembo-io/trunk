# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/aws/postgresql-logfdw.git

# Build extension
RUN cd postgresql-logfdw && \
    make USE_PGXS=1
