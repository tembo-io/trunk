# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build extension
RUN git clone https://github.com/cybertec-postgresql/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
    && git checkout REL_$(perl -E 'print shift =~ s/[.]0\$//gr =~ s/[.]/_/gr' "${EXTENSION_VERSION}") \
    make
