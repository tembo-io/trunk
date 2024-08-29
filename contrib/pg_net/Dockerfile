ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Install dependencies
RUN apt-get update && apt-get install -y libcurl4-openssl-dev

ARG RELEASE=v0.10.0

# Clone repository
RUN git clone https://github.com/supabase/pg_net.git

RUN cd pg_net && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
