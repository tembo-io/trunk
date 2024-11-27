# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/supabase/supautils.git

# Set project version
ARG RELEASE=v1.7.3

# Build extension
RUN cd supautils && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
