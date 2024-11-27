ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION} as build
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
	build-essential \
	libreadline-dev \
	zlib1g-dev \
	flex \
	bison \
	libxml2-dev \
	libxslt-dev \
	libssl-dev \
	libxml2-utils \
	xsltproc \
	ccache \
	libaio1 \
	unzip \
	wget && \
	rm -rf /var/lib/apt/lists/*

# Download and unpack Oracle Instant Client (Basic and SDK)
RUN mkdir /opt/oracle && \
    wget -q https://download.oracle.com/otn_software/linux/instantclient/19800/instantclient-basiclite-linux.x64-19.8.0.0.0dbru.zip && \
    wget -q https://download.oracle.com/otn_software/linux/instantclient/19800/instantclient-sdk-linux.x64-19.8.0.0.0dbru.zip && \
    unzip instantclient-basiclite-linux.x64-19.8.0.0.0dbru.zip -d /opt/oracle && \
    unzip instantclient-sdk-linux.x64-19.8.0.0.0dbru.zip -d /opt/oracle && \
    rm instantclient-basiclite-linux.x64-19.8.0.0.0dbru.zip instantclient-sdk-linux.x64-19.8.0.0.0dbru.zip && \
    echo /opt/oracle/instantclient_19_8 > /etc/ld.so.conf.d/oracle-instantclient.conf && \
    ldconfig && \
    ln -sf /opt/oracle/instantclient_19_8/libclntsh.so.19.1 /opt/oracle/instantclient_19_8/libclntsh.so

# Clone repository
RUN git clone https://github.com/postgres/postgres.git

ARG PG_RELEASE=REL_15_3

# Build extension
RUN cd postgres && \
	git fetch origin ${PG_RELEASE} && \
	git checkout ${PG_RELEASE} && \
	./configure --without-ldap && \
	cd contrib && \
	git clone https://github.com/laurenz/oracle_fdw.git && \
	cd oracle_fdw && \
	make ORACLE_HOME=/opt/oracle/instantclient_19_8 USE_PGXS=1

