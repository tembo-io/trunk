ARG PG_VERSION
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.9

# Extension build dependencies
USER root
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
	pkg-config \
	crossbuild-essential-arm64 \
	crossbuild-essential-amd64 \
	gcc-aarch64-linux-gnu

ARG EXTENSION_NAME
ARG EXTENSION_VERSION
ARG PG_VERSION

# Install Rust and clone & build the extension.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
	&& /root/.cargo/bin/rustup default stable \
	&& git clone --depth 1 --branch v${EXTENSION_VERSION} https://github.com/CrunchyData/${EXTENSION_NAME}.git \
	&& cd ${EXTENSION_NAME} \
	&& cargo pgrx init --pg${PG_VERSION}=/usr/bin/pg_config \
	&& cargo pgrx package