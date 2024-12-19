# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=a6d7f62
RUN git clone https://github.com/aws/postgresql-logfdw.git ${EXTENSION_NAME} \
    && cd ${EXTENSION_NAME} \
    && git checkout ${RELEASE} \
    && make USE_PGXS=1
