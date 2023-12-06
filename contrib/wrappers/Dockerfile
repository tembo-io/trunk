ARG PG_VERSION=15
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.11.0
USER root
# quay.io/coredb/pgrx-builder:pg15-pgrx0.9.7
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
RUN cargo install cargo-pgrx --version 0.11.0 --locked

# Set default Rust version
RUN /root/.cargo/bin/rustup default stable

# Clone repository
RUN git clone https://github.com/supabase/wrappers.git

ARG RELEASE=v0.2.0

# Build the extension
RUN cd wrappers/wrappers && \
	git submodule update --init --recursive && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
	cargo pgrx init --pg15 /usr/bin/pg_config && \
	cargo pgrx package -F all_fdws
