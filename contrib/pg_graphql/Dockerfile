ARG PG_VERSION=17

FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.12.6
USER root

ARG PG_VERSION=17
ARG RELEASE=v1.5.9
RUN apt-get update && apt-get install -y \
      # extension build dependencies
      build-essential \
      ca-certificates \
      curl \
      git \
      libpq-dev \
      libreadline6-dev \
      zlib1g-dev \
    # Install Rust
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    # Set default Rust version
    && /root/.cargo/bin/rustup default stable \
    # Download the source
    && git clone https://github.com/supabase/pg_graphql.git \
    && cd pg_graphql \
    && git fetch origin ${RELEASE} \
    && git checkout ${RELEASE} \
    # Build the extension
    && cargo pgrx init --pg${PG_VERSION}=/usr/bin/pg_config \
    && cargo pgrx package
