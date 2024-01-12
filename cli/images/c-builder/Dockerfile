ARG UBUNTU_VERSION=22.04
FROM quay.io/coredb/ubuntu:${UBUNTU_VERSION}

ADD https://salsa.debian.org/postgresql/postgresql-common/-/raw/master/pgdg/apt.postgresql.org.sh ./

ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC
ARG PG_VERSION=15

RUN apt-get update \
    && apt-get install -y \
      build-essential clang llvm llvm-dev llvm-runtime \
      ca-certificates zip curl wget git software-properties-common flex \
      libicu-dev libxml2 locales ssl-cert \
      pkg-config libssl-dev libclang-dev \
    && apt-get -y purge postgresql-client-common \
    && chmod +x apt.postgresql.org.sh \
    && ./apt.postgresql.org.sh -i -v ${PG_VERSION} \
    && rm -rf apt.postgresql.org.sh /var/cache/apt/* /var/lib/apt/lists/*

USER postgres
WORKDIR /app
