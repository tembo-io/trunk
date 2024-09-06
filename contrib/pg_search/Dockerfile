ARG PG_VERSION=15
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.1
USER root

ARG RELEASE=v0.9.3

# Extension build dependencies
RUN apt-get update && apt-get install -y \
	build-essential

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set default Rust version
RUN /root/.cargo/bin/rustup default 1.80.0-x86_64-unknown-linux-gnu
ARG PG_VERSION=15


# Clone repository
RUN git clone https://github.com/paradedb/paradedb.git && \
	cd paradedb && \
    git fetch --tags && \
    git fetch --depth 1 origin tag ${RELEASE} && \
    git checkout ${RELEASE}


# Build the extension
RUN cd paradedb && \
	cd pg_search && \
    cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config && \
    cargo pgrx package
