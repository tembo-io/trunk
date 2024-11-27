ARG PG_VERSION=17
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

ARG RELEASE=4.5.0
RUN curl -O https://api.pgxn.org/dist/e-maj/${RELEASE}/e-maj-${RELEASE}.zip \
	&& unzip e-maj-${RELEASE}.zip \
	&& cd e-maj-${RELEASE} \
	&& make
