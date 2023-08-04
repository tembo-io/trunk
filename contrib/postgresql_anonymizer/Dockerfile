# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://gitlab.com/dalibo/postgresql_anonymizer.git

# Set project version
ARG RELEASE=1.1.0

# Build extension
RUN cd postgresql_anonymizer && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
