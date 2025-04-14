# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.8

USER root
ARG PG_VERSION
ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Install Rust and download & build the extension.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && /root/.cargo/bin/rustup default stable \
    && curl -O https://api.pgxn.org/dist/${EXTENSION_NAME}/${EXTENSION_VERSION}/${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    && unzip ${EXTENSION_NAME}-${EXTENSION_VERSION}.zip \
    && cd ${EXTENSION_NAME}-${EXTENSION_VERSION} \
    && cargo pgrx init --pg${PG_VERSION}="$(which pg_config)" \
    && cargo pgrx package
WORKDIR /app/${EXTENSION_NAME}-${EXTENSION_VERSION}
