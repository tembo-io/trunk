ARG PG_VERSION=16
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "REL16_$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}")" https://github.com/ossc-db/pg_hint_plan.git \
    && make -C pg_hint_plan
