ARG PG_VERSION
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.11.0

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
	# Set default Rust version
	&& rustup default stable \
	&& rustup component add llvm-tools-preview rustc-dev \
	&& rustup target install x86_64-unknown-linux-gnu \
	&& git clone --depth 1 --branch "v${EXTENSION_VERSION}" https://github.com/tcdi/${EXTENSION_NAME}.git \
	&& cd ${EXTENSION_NAME} \
	&& (cd ${EXTENSION_NAME}c && ./build.sh) \
	&& mv build/bin .cargo \
	&& cd plrust \
	&& env PG_VER=${PG_VERSION} STD_TARGETS="x86_64-postgres-linux-gnu " ./build \
	&& cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config \
	&& cargo pgrx package --features trusted -c /usr/bin/pg_config
