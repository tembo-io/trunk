ARG PG_VERSION=15
ARG TAG=latest

FROM rust:1.82-bookworm as builder

ARG TRUNK_VER=0.15.10

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL sparse
RUN cargo install --version $TRUNK_VER pg-trunk

FROM quay.io/tembo/standard-cnpg:${PG_VERSION}-c92547b

# Install trunk
COPY --from=builder /usr/local/cargo/bin/trunk /usr/bin/trunk

USER root
RUN apt-get update && apt-get install -y \
    jq \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*

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
    libaio1 \
    libbson-dev \
    libgsl-dev \
    && apt-get purge -y --auto-remove libgroonga0 \
    && rm -rf /var/lib/apt/lists/*

# Install zhparser dependency
RUN  wget http://www.xunsearch.com/scws/down/scws-1.2.3.tar.bz2 && \
     tar xvf scws-1.2.3.tar.bz2 && \
     cd scws-1.2.3 && \
     ./configure && \
     make install && \
     cd .. && \
     rm -rf scws-1.2.3.tar.bz2 scws-1.2.3 && \
     ln -s /usr/local/lib/libscws.so /usr/lib/x86_64-linux-gnu/libscws.so

# Install duckdb libs
RUN wget https://github.com/duckdb/duckdb/releases/download/v0.8.1/libduckdb-linux-amd64.zip && \
    unzip libduckdb-linux-amd64.zip && \
    cp libduckdb.so /usr/lib/x86_64-linux-gnu/ && \
    rm -rf libduckdb-linux-amd64.zip libduckdb.so

# Install groonga libs
RUN wget https://packages.groonga.org/source/groonga/groonga-14.1.2.tar.gz \
    && tar xvzf groonga-14.1.2.tar.gz \
    && cd groonga-14.1.2 \
    && ./configure \
    && make -j$(grep '^processor' /proc/cpuinfo | wc -l) \
    && make install \
    && cd .. && rm -rf groonga-14.1.2*

RUN chown -R postgres:postgres $PGDATA && \
    chmod -R 0700 $PGDATA
ENV PGDATA /var/lib/postgresql/data2
RUN mkdir -p $PGDATA
RUN chown -R postgres:postgres $PGDATA && \
    chmod -R 0700 $PGDATA
USER postgres
RUN pg_ctl init
# Set permissive authentication
RUN echo "host all all 0.0.0.0/0 trust" >> ${PGDATA}/pg_hba.conf
COPY postgresql.conf ${PGDATA}/postgresql.conf
COPY trunk-install.sh /usr/local/bin/
