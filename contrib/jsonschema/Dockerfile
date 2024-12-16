ARG PG_VERSION
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.9

ARG EXTENSION_NAME
ARG EXTENSION_VERSION
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y build-essential \
    # Install Rust
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    # Set default Rust version
    && /root/.cargo/bin/rustup default stable \
    # Download the source
    && curl -O https://api.pgxn.org/dist/${EXTENSION_NAME}/${EXTENSION_VERSION}/${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    && unzip ${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    # Build the extension
    && cd ${EXTENSION_NAME}-${EXTENSION_VERSION} && make pgrx-init && make
WORKDIR /app/${EXTENSION_NAME}-${EXTENSION_VERSION}
