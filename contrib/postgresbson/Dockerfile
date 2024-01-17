ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    libbson-1.0-0-dev \
    libmongoc-1.0-0

# Build libbson x.y.z
RUN wget https://github.com/mongodb/libbson/releases/download/x.y.z/libbson-x.y.z.tar.gz \
    && tar -xzvf libbson-x.y.z.tar.gz \
    && cd libbson-x.y.z \
    && ./configure \
    && make -j$(nproc) \
    && make install

# Clone repository
RUN git clone https://github.com/buzzm/postgresbson.git

RUN cd postgresbson && \
    make
