ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "v.$(perl -E 'print shift =~ s/[.]0$//gr' "${EXTENSION_VERSION}")" https://github.com/${EXTENSION_NAME}/${EXTENSION_NAME}.git \
    && printf "EXTENSION = pghydro\nDATA = \$(wildcard pghydro--*.sql)\nPGXS := \$(shell pg_config --pgxs)\ninclude \$(PGXS)\n" > ${EXTENSION_NAME}/Makefile \
    && make -C ${EXTENSION_NAME}
