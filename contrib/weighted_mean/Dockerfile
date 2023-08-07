# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/Kozea/weighted_mean.git

# Build extension
RUN cd weighted_mean && \
    make
