# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    openjdk-11-jdk \
    maven \
    libkrb5-dev \
    libecpg-dev

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
RUN git clone --depth 1 --branch "V$(perl -E 'print shift =~ s/\./_/gr' "${EXTENSION_VERSION}")" https://github.com/tada/${EXTENSION_NAME}.git \
	&& cd ${EXTENSION_NAME} \
    && mvn clean install
