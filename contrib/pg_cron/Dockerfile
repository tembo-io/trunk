ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
ARG RELEASE=v1.5.2

# Clone repository
RUN git clone https://github.com/citusdata/pg_cron.git

RUN cd pg_cron && \
  git fetch origin ${RELEASE} && \
  git checkout ${RELEASE} && \
  make
