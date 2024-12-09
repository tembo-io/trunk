ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/tvondra/lower_quantile.git

RUN cd lower_quantile && \
    make
