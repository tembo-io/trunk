# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/supabase/index_advisor.git

# Set project version
ARG RELEASE=v0.2.0

# Build extension
RUN cd index_advisor && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
