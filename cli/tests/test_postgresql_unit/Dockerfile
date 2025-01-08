ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
# ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "$(perl -E 'print shift =~ s/[.]0$//gr' "${EXTENSION_VERSION}")" https://github.com/df7cb/postgresql-unit.git \
    # Build .sql files to point to the Tembo image share directory before
    # building everything else. Forces these lines to have the values we need:
    # https://github.com/df7cb/postgresql-unit/blob/7.10/Makefile#L44-L45
    && make -C postgresql-unit datadir=/var/lib/postgresql/data/tembo *.sql \
    # Build everything else as usual.
    && make -C postgresql-unit
