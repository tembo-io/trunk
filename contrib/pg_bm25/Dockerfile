ARG PG_VERSION=15
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.11.1
USER root

ARG RELEASE=v0.3.12

# Extension build dependencies
RUN apt-get update && apt-get install -y \
	build-essential

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN cargo install cargo-pgrx --version 0.11.0 --locked

# Set default Rust version
RUN /root/.cargo/bin/rustup default stable


# Clone repository
RUN git clone https://github.com/paradedb/paradedb.git && \
	cd paradedb && \
    git fetch --tags && \
    git fetch --depth 1 origin tag ${RELEASE} && \
    git checkout ${RELEASE}


# Build the extension
RUN cd paradedb && \
	cd pg_bm25 && \
    cargo pgrx init --pg15 /usr/bin/pg_config && \
    cargo pgrx package
