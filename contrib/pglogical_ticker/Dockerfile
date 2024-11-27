# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/enova/pglogical_ticker.git

# Set project version
ARG RELEASE=v1.4.1

# Build extension
RUN cd pglogical_ticker && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
