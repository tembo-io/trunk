ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone and build the extension.
ARG EXTENSION_NAME
ARG EXTENSION_VERSION
ARG RELEASE=e214c79
RUN git clone https://github.com/postgrespro/${EXTENSION_NAME}.git \
    && cd ${EXTENSION_NAME} \
	&& git checkout ${RELEASE} \
	&& make
