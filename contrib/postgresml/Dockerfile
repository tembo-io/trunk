ARG PG_VERSION=17
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.11.3
USER root

ARG RELEASE=v2.9.3
ARG PG_VERSION=17

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

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN $HOME/.cargo/bin/cargo install cargo-pgrx --version=0.11.3 --locked
ARG PG_VERSION=17
RUN $HOME/.cargo/bin/cargo pgrx init --pg$PG_VERSION $(which pg_config)

# Clone repository
RUN git clone https://github.com/postgresml/postgresml.git

# Build the extension
RUN cd postgresml && \
	git submodule update --init --recursive && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
	cd pgml-extension && \
	cargo update && \
    cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config && \
    cargo pgrx package
