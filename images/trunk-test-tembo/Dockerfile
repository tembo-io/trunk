FROM quay.io/tembo/tembo-pg-cnpg:15.3.0-5-429c3cd

USER root
RUN apt-get update && apt-get install -y \
    jq \
    curl \
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
