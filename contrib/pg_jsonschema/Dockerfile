ARG PG_VERSION
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.6

ARG PG_VERSION
ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && /root/.cargo/bin/rustup default stable \
    && git clone --depth 1 --branch v${EXTENSION_VERSION} https://github.com/supabase/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && cargo pgrx init --pg${PG_VERSION}=/usr/bin/pg_config \
    && cargo pgrx package
