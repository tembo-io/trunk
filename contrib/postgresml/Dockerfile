ARG PG_VERSION=16
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.11.0
USER root

ARG RELEASE=v2.7.13
ARG PG_VERSION=16

# Extension build dependencies
RUN apt-get update && apt-get install -y \
	build-essential \
	clang \
	libopenblas-dev \
	libssl-dev \
	bison \
	flex \
	pkg-config \
	cmake \
	libreadline-dev \
	libz-dev \
	tzdata \
	sudo \
	libpq-dev \
	libclang-dev \
	postgresql-${PG_VERSION} \
	postgresql-server-dev-${PG_VERSION} \
	python3 \
	python3-pip \
	libpython3-dev \
	lld

RUN rustup default 1.78.0-x86_64-unknown-linux-gnu

# Clone repository
RUN git clone https://github.com/postgresml/postgresml.git

# Build the extension
RUN cd postgresml && \
	git submodule update --init --recursive && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
	cd pgml-extension && \
    cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config && \
    cargo pgrx package
