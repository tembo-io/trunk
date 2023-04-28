ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
ARG PG_VERSION=15
# When Rust 1.68.0 is released, use this instead of nightly
# for enabling sparse registry, a feature required for
# multi-stage build to work.
#
# ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
ENV PATH="/var/lib/postgresql/.cargo/bin:${PATH}"

# Install tcdi/pgrx
ARG PGRX_VERSION=0.8.2
RUN cargo --version && cargo install -Z sparse-registry cargo-pgrx --version "${PGRX_VERSION}"
RUN cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config
