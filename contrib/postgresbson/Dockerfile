ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    libbson-dev
# Clone repository
RUN git clone https://github.com/buzzm/postgresbson.git
RUN mkdir -p ~/projects/bson/include
# For some reason these need to be in ~/projects/bson/include/ and postgresbson/
RUN cp -r /usr/include/libbson-1.0/* ~/projects/bson/include/
RUN cp -r /usr/include/libbson-1.0/* postgresbson/

# Replacing BSON_SHLIB with the path to libbson-1.0.so (https://github.com/buzzm/postgresbson/blob/5b442c42642075c43edbfc41c1e2e7e6d1034758/Makefile#L9)
RUN cd postgresbson && make BSON_SHLIB=/usr/lib/x86_64-linux-gnu/libbson-1.0.so
