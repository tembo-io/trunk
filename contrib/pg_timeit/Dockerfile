ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
# ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG RELEASE=d83ab65
RUN git clone https://github.com/joelonsql/pg-timeit.git \
    && cd pg-timeit \
    && git checkout ${RELEASE} \
    && make
