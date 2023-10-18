ARG UBUNTU_VERSION=22.04
FROM quay.io/coredb/ubuntu:${UBUNTU_VERSION}

ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC
ARG PG_VERSION=15

RUN apt-get update && \
    apt-get install -y \
    curl unzip gpg wget git software-properties-common flex \
    && echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list \
    && wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - \
    && apt-get update  \
    && apt-get install --allow-downgrades -y \
      postgresql-${PG_VERSION} \
      postgresql-server-dev-${PG_VERSION} \
      pkg-config \
      libssl-dev \
      build-essential \
      libclang-14-dev \
      clang-14 \
    # We need to make sure that only clang-14 is installed in the image before
    # it tries to compile pgrx, as having multiple versions cause issues
    # per https://github.com/pgcentralfoundation/pgrx/issues/1298
    #
    # Could instead do this in the pgrx builder image
    && apt-get autoremove -y clang-13 lldb-13 lld-13 clang-15 lldb-15 lld-15 \
    && rm -rf /var/lib/apt/lists/*

USER postgres
WORKDIR /app
