FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/var/lib/postgresql/.cargo/bin:${PATH}"

# Install tcdi/pgrx
ARG PGRX_VERSION=0.8.2
RUN cargo --version && cargo install cargo-pgrx --version "${PGRX_VERSION}"
RUN cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config
