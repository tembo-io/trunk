ARG PG_VERSION=17
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.6
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
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
	python3-dev \
	clang \
	cmake \
	libclang-dev \
	libopenblas-dev \
	libpython3-dev \
	pkg-config

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN cargo install cargo-pgrx --version 0.12.6 --locked

# Set default Rust version
RUN /root/.cargo/bin/rustup default stable

# Clone repository
RUN git clone https://github.com/supabase/wrappers.git

ARG RELEASE=v0.4.3

# Build the extension
RUN cd wrappers/wrappers && \
	git submodule update --init --recursive && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
	cargo pgrx init --pg17 /usr/bin/pg_config && \
	cargo pgrx package -F all_fdws
