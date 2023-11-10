# Set the PostgreSQL version
ARG PG_VERSION=15
FROM quay.io/coredb/c-builder:pg${PG_VERSION}
USER root

# Extension build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    libreadline-dev \
    zlib1g-dev \
    flex \
    bison \
    libxml2-dev \
    libxslt-dev \
    libssl-dev \
    libxml2-utils \
    xsltproc \
    ccache \
    python3 \
    python3-dev \
    python3-setuptools \
    python3-pip && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Define the version of E-Maj to install
ARG EMAJ_VERSION=4.3.1

# Download and unpack E-Maj
RUN wget https://api.pgxn.org/dist/e-maj/${EMAJ_VERSION}/e-maj-${EMAJ_VERSION}.zip && \
    unzip e-maj-${EMAJ_VERSION}.zip && \
    rm e-maj-${EMAJ_VERSION}.zip

# Copy the necessary files to the PostgreSQL extension directory
RUN SHAREDIR=$(pg_config --sharedir) && \
    cp e-maj-${EMAJ_VERSION}/emaj.control ${SHAREDIR}/extension/ && \
    cp e-maj-${EMAJ_VERSION}/sql/emaj--* ${SHAREDIR}/extension/ && \
    rm -r e-maj-${EMAJ_VERSION}
