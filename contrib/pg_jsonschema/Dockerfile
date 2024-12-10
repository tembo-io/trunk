ARG PG_VERSION
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.6

ARG PG_VERSION=17
ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && /root/.cargo/bin/rustup default stable \
    && git clone https://github.com/supabase/pg_jsonschema.git \
    && cd pg_jsonschema \
    && git checkout v${EXTENSION_VERSION} \
    && cargo pgrx init --pg${PG_VERSION}=/usr/bin/pg_config \
    && cargo pgrx package
