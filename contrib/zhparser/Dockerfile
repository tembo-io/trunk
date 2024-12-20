ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Install dependency.
USER root
RUN wget http://www.xunsearch.com/scws/down/scws-1.2.3.tar.bz2 \
    && tar xvf scws-1.2.3.tar.bz2 \
    && cd scws-1.2.3 \
    && ./configure \
    && make install \
    && cd .. \
    && rm -rf scws-1.2.3.tar.bz2 scws-1.2.3 \
    && ln -s /usr/local/lib/libscws.so /usr/lib/x86_64-linux-gnu/libscws.so

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "V$(perl -E 'print shift =~ s/[.]0$//gr' "${EXTENSION_VERSION}")" https://github.com/amutu/${EXTENSION_NAME}.git \
    && make -C ${EXTENSION_NAME}
