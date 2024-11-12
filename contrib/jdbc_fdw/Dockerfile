# Set PostgreSQL version
ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root




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
# Clone postgres repository
ARG PG_RELEASE=REL_15_3
RUN git clone  --depth 1 --branch ${PG_RELEASE} https://github.com/postgres/postgres.git
RUN cd postgres && \
	./configure

#Clone jdbc repository
ARG JDBC_RELEASE=v0.3.1
RUN cd postgres/contrib && \
    git clone --depth 1 --branch ${JDBC_RELEASE} https://github.com/pgspider/jdbc_fdw.git


RUN cd postgres/contrib/jdbc_fdw && \
    make clean && \
    make

    



