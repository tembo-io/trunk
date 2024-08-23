ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
ARG EXTENSION_VERSION=v1.6.4

# Clone repository
RUN git clone https://github.com/citusdata/pg_cron.git

RUN cd pg_cron && \
  git fetch origin ${EXTENSION_VERSION} && \
  git checkout ${EXTENSION_VERSION} && \
  make
