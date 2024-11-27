ARG PG_VERSION=17
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.5
USER root

ARG RELEASE=0.1.3

# Extension build dependencies
RUN apt-get update && apt-get install -y build-essential \
    # Install Rust
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    # Set default Rust version
    && /root/.cargo/bin/rustup default stable \
    # Download the source
    && curl -O https://api.pgxn.org/dist/jsonschema/${RELEASE}/jsonschema-${RELEASE}.zip \
    && unzip jsonschema-${RELEASE}.zip \
    # Build the extension
    && cd jsonschema-${RELEASE} && make pgrx-init && make
