# Set PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    librdkafka-dev \
    zlib1g-dev

# Clone repository
RUN git clone https://github.com/adjust/kafka_fdw.git

# Build extension
RUN cd kafka_fdw && \
    make
