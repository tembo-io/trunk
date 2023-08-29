ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Clone repository
RUN git clone https://github.com/pgvector/pgvector.git

ARG RELEASE=v0.5.0

# Build extension
RUN cd pgvector && \
	git fetch origin ${RELEASE} && \
	git checkout ${RELEASE} && \
	make
