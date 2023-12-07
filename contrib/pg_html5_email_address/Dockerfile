# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Clone repository
RUN git clone https://github.com/bigsmoke/pg_html5_email_address.git

ARG RELEASE=v1.2.2

# Build extension
RUN cd pg_html5_email_address && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make

