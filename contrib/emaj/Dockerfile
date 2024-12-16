ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN curl -O https://api.pgxn.org/dist/e-maj/${EXTENSION_VERSION}/e-maj-${EXTENSION_VERSION}.zip \
	&& unzip e-maj-${EXTENSION_VERSION}.zip \
	&& cd e-maj-${EXTENSION_VERSION} \
	&& perl -i -pe 's/#directory/directory/' ${EXTENSION_NAME}.control \
	&& make
WORKDIR /app/e-maj-${EXTENSION_VERSION}
