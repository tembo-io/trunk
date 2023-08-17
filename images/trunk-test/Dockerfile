FROM rust:1.70.0-slim-bookworm as builder
ARG TRUNK_VER=0.8.3
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL sparse
RUN apt update && apt install -y pkg-config libssl-dev
RUN cargo install --version $TRUNK_VER pg-trunk

FROM quay.io/coredb/ubuntu:22.04

COPY --from=builder /usr/local/cargo/bin/trunk /usr/bin/trunk

ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC

# Set the postgres user's permissions
RUN set -eux; \
	groupadd -r postgres --gid=999; \
	useradd -r -g postgres --uid=999 --home-dir=/var/lib/postgresql --shell=/bin/bash postgres; \
	mkdir -p /var/lib/postgresql; \
	chown -R postgres:postgres /var/lib/postgresql

RUN apt-get update && apt-get install -y \
    curl \
    jq \
    ca-certificates \
    gnupg \
    lsb-release \
    libreadline-dev \
    zlib1g-dev \
    libpq-dev \
    wget \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

STOPSIGNAL SIGINT

ENV PGDATA /var/lib/postgresql/data
ENV PG_VERSION 15.3
ENV PG_MAJOR 15
ENV PATH $PATH:/usr/lib/postgresql/$PG_MAJOR/bin

RUN set -eux; \
	apt-get update; apt-get install -y --no-install-recommends locales; rm -rf /var/lib/apt/lists/*; \
	localedef -i en_US -c -f UTF-8 -A /usr/share/locale/locale.alias en_US.UTF-8
ENV LANG en_US.utf8

RUN mkdir /docker-entrypoint-initdb.d

RUN sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list' && \
    wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - && \
    apt-get update && \
    apt-get upgrade -y

# Install extension dependencies
RUN apt-get update && apt-get install -y \
    libmysqlclient-dev \
    libtcl8.6 \
    libgeos-dev \
    libproj-dev \
    libjson-c-dev \
    libprotobuf-c-dev \
    libxml2-dev \
    libboost-serialization1.74-dev \
    libhiredis-dev \
    libsybdb5 \
    libpython3.10-dev \
    r-base-core \
    openssl \
    liblz4-1 \
    libpcre2-8-0 \
    libuuid1 \
    libgroonga0 \
    libopenblas0-pthread \
    libcurl4 \
    libjson-c5 \
    libsodium23 \
    libgcc-s1 \
    libselinux1 \
    librdkafka1 \
    libgdal30 \
    libcrypt1 \
    liburiparser1 \
    libfreetype6 \
    libzstd1 \
    zlib1g \
    libperl5.34 \
    libgomp1 \
    libssl3 \
    libsfcgal1 \
    openjdk-11-jdk \
    && rm -rf /var/lib/apt/lists/*

RUN ln -s /usr/lib/jvm/java-11-openjdk-amd64/lib/server/libjvm.so /usr/lib/x86_64-linux-gnu/libjvm.so

# Build Postgres from source
RUN curl https://ftp.postgresql.org/pub/source/v15.3/postgresql-15.3.tar.bz2 -o postgresql-15.3.tar.bz2
RUN tar xf postgresql-${PG_VERSION}.tar.bz2
WORKDIR postgresql-${PG_VERSION}
RUN ./configure --prefix=/usr/lib/postgresql/${PG_MAJOR} --datarootdir=/usr/share
RUN make
RUN make install
RUN cd .. && rm postgresql-${PG_VERSION}.tar.bz2

COPY ./postgresql.conf /usr/share/postgresql/${PG_MAJOR}/postgresql.conf.sample

# Remove pre-installed pg_config
RUN rm /usr/bin/pg_config

# cache extensions and shared libraries
RUN mkdir /tmp/pg_sharedir && \
        mkdir /tmp/pg_pkglibdir && \
        cp -r $(pg_config --sharedir)/* /tmp/pg_sharedir && \
        cp -r $(pg_config --pkglibdir)/* /tmp/pg_pkglibdir
RUN mkdir -p /var/run/postgresql && chmod 775 /var/run/postgresql
RUN mkdir -p /usr/share/postgresql/${PG_MAJOR}/extension && chmod 775 /usr/share/postgresql/${PG_MAJOR}/extension
RUN chown -R postgres:postgres /usr/lib/postgresql/${PG_MAJOR} /usr/share/postgresql/extension
COPY docker-entrypoint.sh /usr/local/bin/
COPY trunk-install.sh /usr/local/bin/
ENTRYPOINT ["docker-entrypoint.sh"]

USER postgres
CMD ["postgres"]
