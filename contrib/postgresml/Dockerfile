FROM quay.io/coredb/pgrx-builder:pg15-pgrx0.11.2

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
 	lld

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN cargo install cargo-pgrx --version 0.11.2 --locked
# Set default Rust version
RUN /root/.cargo/bin/rustup default stable

# Clone repository
RUN git clone https://github.com/postgresml/postgresml.git

# python build dependencies
RUN add-apt-repository ppa:deadsnakes/ppa && \
	apt update && sudo apt install -y libpython3.12-dev
 
RUN pip3 install xgboost

ARG RELEASE=v2.8.1
# Build the extension
RUN cd postgresml && \
	git submodule update --init --recursive && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
	cd pgml-extension && \
    cargo pgrx init --pg15 /usr/bin/pg_config && \
    cargo pgrx package
