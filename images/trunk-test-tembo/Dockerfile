FROM quay.io/tembo/tembo-pg-cnpg:15.3.0-5-c9fde12

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
    libaio1 \
    && rm -rf /var/lib/apt/lists/*
RUN ln -s /usr/lib/jvm/java-11-openjdk-amd64/lib/server/libjvm.so /usr/lib/x86_64-linux-gnu/libjvm.so
RUN wget https://download.oracle.com/otn_software/linux/instantclient/1920000/instantclient-basiclite-linux.x64-19.20.0.0.0dbru.zip && \
    unzip instantclient-basiclite-linux.x64-19.20.0.0.0dbru.zip && \
    cp instantclient_19_20/libclntsh.so.19.1 /usr/lib/x86_64-linux-gnu/ && \
    cp instantclient_19_20/libnnz19.so /usr/lib/x86_64-linux-gnu/ && \
    cp instantclient_19_20/libclntshcore.so.19.1 /usr/lib/x86_64-linux-gnu/ && \
    rm -rf instantclient_19_20 && \
    rm instantclient-basiclite-linux.x64-19.20.0.0.0dbru.zip

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
