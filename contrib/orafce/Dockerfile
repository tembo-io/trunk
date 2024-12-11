ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

# Build extension
RUN git clone https://github.com/${EXTENSION_NAME}/${EXTENSION_NAME}.git \
    && cd {EXTENSION_NAME} \
    && git checkout VERSION_$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}") \
    && make
