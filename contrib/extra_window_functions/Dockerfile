ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Clone repository
RUN git clone https://github.com/xocolatl/extra_window_functions.git

ARG RELEASE=v1.0

RUN cd extra_window_functions && \
    git fetch origin ${RELEASE} && \
    git checkout ${RELEASE} && \
    make
