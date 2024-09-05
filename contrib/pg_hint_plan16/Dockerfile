ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/ossc-db/pg_hint_plan.git

ARG RELEASE=REL16_1_6_0

RUN cd pg_hint_plan && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
