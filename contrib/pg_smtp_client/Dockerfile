ARG PG_VERSION=15
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.11.4
USER root

ARG RELEASE=v0.1.0

# Extension build dependencies
RUN apt-get update && apt-get install -y \
	build-essential

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN cargo install cargo-pgrx --version 0.11.4 --locked

# Set default Rust version
RUN /root/.cargo/bin/rustup default stable

# Clone repository
RUN git clone --depth 1 --branch ${RELEASE} https://github.com/brianpursley/pg_smtp_client

# Build the extension
RUN cd pg_smtp_client && \
    cargo pgrx init --pg15 /usr/bin/pg_config && \
    cargo pgrx package
