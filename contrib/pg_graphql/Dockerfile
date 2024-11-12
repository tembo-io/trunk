ARG PG_VERSION=17

FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx0.11.2
USER root

# extension build dependencies
RUN apt-get update && apt-get install -y \
  build-essential \
  ca-certificates \
  curl \
  git \
  libpq-dev \
  libreadline6-dev \
  zlib1g-dev

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set default Rust version
RUN /root/.cargo/bin/rustup default stable && /root/.cargo/bin/rustup override set 1.75.0

RUN git clone https://github.com/supabase/pg_graphql.git

ARG RELEASE=v1.4.4

# build
RUN cd pg_graphql && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
    cargo pgrx init --pg15 /usr/bin/pg_config && \
    cargo pgrx package
