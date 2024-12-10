ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# ARG EXTENSION_NAME
ARG EXTENSION_VERSION
ARG REVISION=d83ab65

RUN git clone --depth 1 --branch "${REVISION}" https://github.com/joelonsql/pg-timeit.git \
    && cd pg-timeit && \
    && make
