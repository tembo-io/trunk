ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

ARG RELEASE=3.2.3
RUN curl -O https://api.pgxn.org/dist/plv8/${RELEASE}/plv8-${RELEASE}.zip \
	&& unzip plv8-${RELEASE}.zip \
	&& cd plv8-${RELEASE} \
	&& make
