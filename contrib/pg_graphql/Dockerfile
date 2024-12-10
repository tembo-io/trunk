ARG PG_VERSION
FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.6

# extension build dependencies
USER root
RUN apt-get update && apt-get install -y \
    build-essential \
    ca-certificates \
    curl \
    git \
    libpq-dev \
    libreadline6-dev \
    zlib1g-dev

ARG PG_VERSION
ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build the extension
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && /root/.cargo/bin/rustup default stable \
    && git clone --depth 1 --branch v${EXTENSION_VERSION} https://github.com/supabase/${EXTENSION_NAME} \
    && cd ${EXTENSION_NAME} \
    && cargo pgrx init --pg${PG_VERSION} /usr/bin/pg_config \
    && cargo pgrx package --features
