ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    libkrb5-dev

# Clone repository
RUN git clone https://github.com/EnterpriseDB/pg_failover_slots.git

ARG RELEASE=v1.0.1

RUN cd pg_failover_slots && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
