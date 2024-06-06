ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Clone repository
RUN git clone https://github.com/pgvector/pgvector.git

ARG EXTENSION_VERSION=v0.7.1

# Build extension
RUN cd pgvector && \
	git fetch origin v${EXTENSION_VERSION} && \
	git checkout v${EXTENSION_VERSION} && \
	make
