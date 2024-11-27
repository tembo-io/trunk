ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/MigOpsRepos/pg_dbms_job.git

ARG RELEASE=v1.5

RUN cd pg_dbms_job && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
