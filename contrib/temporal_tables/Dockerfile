# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/arkhipov/temporal_tables.git

# Set project version
ARG RELEASE=v1.2.1

# Build extension
RUN cd temporal_tables && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
