ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
ARG PG_VERSION=15

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/var/lib/postgresql/.cargo/bin:${PATH}"

# Install pgrx
ARG PGRX_VERSION=0.8.2
RUN set -ex; \
    if [ "$PGRX_VERSION" = "0.13.1" ] || [ "$PGRX_VERSION" = "0.13.0" ]; then \
        # https://github.com/pgcentralfoundation/pgrx/issues/2009
        cargo --version && cargo install cargo-pgrx --version "${PGRX_VERSION}" --locked; \
    else \
        cargo --version && cargo install cargo-pgrx --version "${PGRX_VERSION}"; \
    fi; \
    cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config
