ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG RELEASE=1.0.0

# Extension build dependencies
RUN curl -O https://api.pgxn.org/dist/envvar/${RELEASE}/envvar-${RELEASE}.zip \
    && unzip envvar-${RELEASE}.zip \
    && cd envvar-${RELEASE} && make
