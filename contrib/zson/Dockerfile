ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
	build-essential

# Clone repository
RUN git clone https://github.com/postgrespro/zson

RUN cd zson && make