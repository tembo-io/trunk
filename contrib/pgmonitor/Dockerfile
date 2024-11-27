# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/CrunchyData/pgmonitor-extension.git

# Set project version
ARG EXTENSION_VERSION

# Build extension
RUN cd pgmonitor-extension && \
    git fetch origin v${EXTENSION_VERSION} && \
    git checkout v${EXTENSION_VERSION} && \
    make
