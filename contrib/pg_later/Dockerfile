ARG PG_VERSION=15
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.11.1
USER root

ARG RELEASE=v0.7.6

# Extension build dependencies
RUN apt-get update && apt-get install -y \
	build-essential

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set default Rust version
RUN /root/.cargo/bin/rustup default 1.75.0-x86_64-unknown-linux-gnu
ARG PG_VERSION=15


# Install pgmq
RUN git clone https://github.com/tembo-io/pgmq.git && \
	cd pgmq/pgmq-extension && \
	make && \
	make install && \
	cd .. && rm -rf pgmq


# Clone repository
RUN git clone https://github.com/tembo-io/pg_later.git

# Build the extension
RUN cd pg_later && \
    cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config && \
    cargo pgrx package
