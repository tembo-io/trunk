ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/eulerto/pg_similarity.git

RUN cd pg_similarity && \
    make
