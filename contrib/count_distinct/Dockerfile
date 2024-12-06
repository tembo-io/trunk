# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/tvondra/count_distinct.git 

# Set project version
ARG RELEASE=8f4883a

# Build extension
RUN cd count_distinct && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
