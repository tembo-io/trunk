# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Clone repository
RUN git clone https://github.com/postgres/postgres.git

ARG PG_RELEASE=REL_15_3

# Extension build dependencies
RUN apt-get update && apt-get install -y \
		build-essential \
		libreadline-dev \
		zlib1g-dev \
		flex bison \
		libxml2-dev \
		libxslt-dev \
		libssl-dev \
		libxml2-utils \
		xsltproc \
		ccache \
    openjdk-11-jdk

RUN mkdir -p /usr/lib64 && ln -s /usr/lib/jvm/java-1.11.0-openjdk-amd64/lib/server/libjvm.so /usr/lib64/libjvm.so

RUN cd postgres && \
	git fetch origin ${PG_RELEASE} && \
	git checkout ${PG_RELEASE} && \
	./configure

# TODO: git checkout specific release version of jdbc_fdw
RUN cd postgres/contrib && \
		git clone https://github.com/pgspider/jdbc_fdw.git

RUN cd postgres/contrib/jdbc_fdw && \
    make clean && \
    make

    



