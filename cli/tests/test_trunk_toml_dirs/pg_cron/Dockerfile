FROM quay.io/coredb/c-builder:pg15

ARG VERSION=1.5.2

RUN git clone https://github.com/citusdata/pg_cron.git
RUN cd pg_cron && \
  git fetch origin v${VERSION} && \
  git checkout v${VERSION} && \
  make
