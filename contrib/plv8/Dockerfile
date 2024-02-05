ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

ARG RELEASE=v3.2.2
RUN git clone https://github.com/plv8/plv8.git \
	&& cd plv8 \
	&& git fetch origin ${RELEASE} \
	&& git checkout ${RELEASE} \
	&& make install \
	&& ls -lah /usr/share/postgresql/15/extension
