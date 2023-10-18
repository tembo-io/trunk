ARG PG_VERSION=15
ARG RELEASE=v1.4.1

FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.9.7
USER root

# extension build dependencies
RUN apt-get update && apt-get install -y \
  build_deps \
  build-essential \
  ca-certificates \
  curl \
  git \
  libpq-dev \
  libreadline6-dev \
  postgresql-server-dev-${PG_MAJOR} \
  zlib1g-dev

# build
RUN cd pg_graphql && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
    cargo pgrx init --pg15 /usr/bin/pg_config && \
    cargo pgrx package
