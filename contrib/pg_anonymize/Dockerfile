# Set PostgreSQL version
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
# ARG EXTENSION_VERSION
ARG REVISION=692b354

# Clone repository and build the extension.
RUN git clone https://github.com/rjuju/${EXTENSION_NAME}.git ${RELEASE} \
	&& cd ${EXTENSION_NAME} \
    && git checkout ${REVISION} \
	&& make
