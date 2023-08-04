ARG PG_VERSION=15
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.9.7
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
	pkg-config \
	crossbuild-essential-arm64 \
	crossbuild-essential-amd64 \
	gcc-aarch64-linux-gnu

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set default Rust version
RUN rustup default stable

RUN rustup component add llvm-tools-preview rustc-dev && \
	rustup target install x86_64-unknown-linux-gnu

# Clone repository
RUN git clone https://github.com/tcdi/plrust.git

ARG RELEASE=v1.2.3

# Build the extension
RUN cd plrust && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
	cd plrustc && \
	./build.sh && \
	cd .. && \
	pwd && \
	ls -la

RUN	mv /app/plrust/build/bin/plrustc /app/plrust/.cargo/bin && \
	cd /app/plrust/plrust && \
	PG_VER=15 && \
	STD_TARGETS="x86_64-postgres-linux-gnu " && \
	./build && \
    cargo pgrx init --pg15 /usr/bin/pg_config && \
    cargo pgrx package --features trusted -c /usr/bin/pg_config
